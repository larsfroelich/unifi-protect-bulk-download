use crate::{UnifiProtectCamera, UnifiProtectServer};
use reqwest::Client;
use crate::camera::UnifiProtectCameraSimple;

impl UnifiProtectServer {
    pub async fn fetch_cameras(&mut self, require_detailed_cameras : bool) -> Result<(), String> {
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(&(self.uri.to_string() + "/proxy/protect/api/cameras"))
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Failed to make fetch cameras request");

        // Something went wrong with the login call, possibly a controller reboot or failure.
        if !response.status().is_success() {
            println!("Failed to fetch cameras: {}", response.status());
            return Err(String::from("Failed to make cameras request!"));
        }

        // fetch the raw JSON text
        let cameras_raw_text = response.text().await;
        if cameras_raw_text.is_err() {
            return Err(format!("Failed to parse camera-data: {}", cameras_raw_text.err().unwrap().to_string()));
        }

        // attempt to parse the most basic camera data
        let parsed_cameras_simple_result = serde_json::from_str::<Vec<UnifiProtectCameraSimple>>(cameras_raw_text.as_ref().unwrap());
        if parsed_cameras_simple_result.is_err() {
            return Err(format!("Failed to parse camera-data: {}", parsed_cameras_simple_result.err().unwrap().to_string()));
        }
        self.cameras_simple = parsed_cameras_simple_result.unwrap();

        // attempt to parse complete camera data
        let parsed_cameras_result = serde_json::from_str::<Vec<UnifiProtectCamera>>(cameras_raw_text.as_ref().unwrap());
        if !parsed_cameras_result.is_err() {
            self.cameras = parsed_cameras_result.unwrap();
        }else if require_detailed_cameras {
            return Err(format!("Failed to parse camera-data: {}", parsed_cameras_result.err().unwrap().to_string()));
        }else{
            println!("Warning: Unable to parse complete set of camera data - data formats dont match");
        }

        Ok(())
    }
}
