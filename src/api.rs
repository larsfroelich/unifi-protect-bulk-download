// api.rs - Unifi Protect API client
// Implements the 2025/2026 Unifi Protect API for authentication, camera listing, and video download.
// The old unifi_protect crate used deprecated endpoints - this module uses the current API format.

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/// Represents a camera from the Unifi Protect API
/// Only includes fields we actually need - the API returns much more
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub mac: String,
    #[serde(default)]
    pub is_connected: bool,
    #[serde(rename = "type")]
    #[allow(dead_code)]  // Useful for debugging, kept for future features
    pub camera_type: Option<String>,
}

/// Login request payload
#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
    remember: bool,
}

/// Main API client for Unifi Protect
/// Uses a cookie jar for session management after login
pub struct UnifiProtectClient {
    client: Client,
    base_url: String,
}

impl UnifiProtectClient {
    /// Create a new client for the given Unifi Protect server URL
    /// The URL should be like "https://192.168.1.1" or "https://protect.example.com"
    pub fn new(base_url: &str) -> Self {
        // Build a client with cookie storage for session management
        // Also accept invalid certs since many Unifi setups use self-signed
        let client = Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        // Clean up the base URL - remove trailing slash
        let base_url = base_url.trim_end_matches('/').to_string();

        Self { client, base_url }
    }

    /// Authenticate with the Unifi Protect server
    /// This sets session cookies that will be used for subsequent requests
    pub async fn login(&self, username: &str, password: &str) -> Result<(), String> {
        let login_url = format!("{}/api/auth/login", self.base_url);

        let login_req = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
            remember: true,
        };

        let response = self.client
            .post(&login_url)
            .json(&login_req)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err("Invalid username or password".to_string()),
            status => Err(format!("Login failed with status: {}", status)),
        }
    }

    /// Fetch the list of cameras from the NVR
    /// Returns a Vec of Camera structs with basic info
    pub async fn get_cameras(&self) -> Result<Vec<Camera>, String> {
        let cameras_url = format!("{}/proxy/protect/api/cameras", self.base_url);

        let response = self.client
            .get(&cameras_url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch cameras: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch cameras: HTTP {}", response.status()));
        }

        let cameras: Vec<Camera> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse camera list: {}", e))?;

        Ok(cameras)
    }

    /// Download video footage for a camera within a time range
    ///
    /// The Unifi Protect API uses a two-step process:
    /// 1. PREPARE - Request the video to be prepared for download
    /// 2. DOWNLOAD - Actually download the prepared video
    ///
    /// # Arguments
    /// * `camera` - The camera to download from
    /// * `start_ms` - Start timestamp in milliseconds
    /// * `end_ms` - End timestamp in milliseconds
    /// * `display_filename` - Human-readable filename for the download
    /// * `output_path` - Where to save the downloaded video
    ///
    /// # Returns
    /// * `Ok(true)` - Video downloaded successfully
    /// * `Ok(false)` - No recording found for this time range
    /// * `Err(...)` - An error occurred
    pub async fn download_video(
        &self,
        camera: &Camera,
        start_ms: i64,
        end_ms: i64,
        display_filename: &str,
        output_path: &str,
    ) -> Result<bool, String> {
        // Step 1: PREPARE - Request video preparation
        let prepare_result = self.prepare_video(camera, start_ms, end_ms, display_filename).await?;

        let prepared_filename = match prepare_result {
            Some(name) => name,
            None => return Ok(false), // No recording found
        };

        // Step 2: DOWNLOAD - Fetch the prepared video
        self.fetch_prepared_video(&camera.id, &prepared_filename, output_path).await
    }

    /// Step 1: Prepare video for download
    /// Returns the filename to use for download, or None if no recording exists
    async fn prepare_video(
        &self,
        camera: &Camera,
        start_ms: i64,
        end_ms: i64,
        display_filename: &str,
    ) -> Result<Option<String>, String> {
        let encoded_filename = urlencoding::encode(display_filename);

        // Build prepare URL with all required parameters
        let prepare_url = format!(
            "{}/proxy/protect/api/video/prepare?camera={}&channel=0&start={}&end={}&filename={}&fps=0&lens=0&type=rotating",
            self.base_url, camera.id, start_ms, end_ms, encoded_filename
        );

        let response = self.client
            .get(&prepare_url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Prepare request failed: {}", e))?;

        let status = response.status();

        match status {
            StatusCode::OK => {
                // Parse the response to get the prepared filename
                let body: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse prepare response: {}", e))?;

                if let Some(filename) = body.get("fileName").and_then(|v| v.as_str()) {
                    Ok(Some(filename.to_string()))
                } else {
                    Err("Prepare response missing fileName".to_string())
                }
            }
            StatusCode::NOT_FOUND => {
                // Check for "no recording" error
                if let Some(error_code) = response.headers().get("x-error-code") {
                    if error_code.to_str().unwrap_or("") == "kRecordingNotFound" {
                        return Ok(None);
                    }
                }
                Ok(None)
            }
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(format!("Prepare failed with status: {} - {}", status, body))
            }
        }
    }

    /// Step 2: Download the prepared video file
    async fn fetch_prepared_video(
        &self,
        camera_id: &str,
        filename: &str,
        output_path: &str,
    ) -> Result<bool, String> {
        let encoded_filename = urlencoding::encode(filename);

        // Download URL needs both camera ID and filename
        let download_url = format!(
            "{}/proxy/protect/api/video/download?camera={}&filename={}",
            self.base_url, camera_id, encoded_filename
        );

        let response = self.client
            .get(&download_url)
            .send()
            .await
            .map_err(|e| format!("Download request failed: {}", e))?;

        match response.status() {
            StatusCode::OK => {
                self.stream_to_file(response, output_path).await?;
                Ok(true)
            }
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(format!("Download failed with status: {} - {}", status, body))
            }
        }
    }

    /// Stream a response body to a file
    /// Uses streaming to handle large video files without loading into memory
    async fn stream_to_file(&self, response: reqwest::Response, path: &str) -> Result<(), String> {
        let mut file = File::create(path)
            .await
            .map_err(|e| format!("Failed to create output file: {}", e))?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Error reading response: {}", e))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| format!("Error writing to file: {}", e))?;
        }

        file.flush()
            .await
            .map_err(|e| format!("Error flushing file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_cleanup() {
        let client = UnifiProtectClient::new("https://example.com/");
        assert_eq!(client.base_url, "https://example.com");

        let client2 = UnifiProtectClient::new("https://example.com");
        assert_eq!(client2.base_url, "https://example.com");
    }
}
