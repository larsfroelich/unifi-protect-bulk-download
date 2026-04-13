use crate::UnifiProtectServer;
use reqwest::Client;
use serde_json::json;

impl UnifiProtectServer {
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), &str> {
        // Already logged in?
        if self.headers.contains_key("Cookie") && self.headers.contains_key("X-CSRF-Token") {
            return Ok(());
        }

        // Make sure we have a CSRF token, or get one if needed.
        self.acquire_csrf_token().await;

        // Log in
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(&(self.uri.clone() + "/api/auth/login"))
            .headers(self.headers.clone())
            .json(&json!({
                "password": password,
                "rememberMe": true,
                "username": username,
                "token": ""
            }))
            .send()
            .await
            .expect("Failed to make login request");

        // Something went wrong with the login call, possibly a controller reboot or failure.
        if !response.status().is_success() {
            println!(
                "Failed to log in: {} {:?} {:?}",
                response.status(),
                response.headers(),
                response.url()
            );
            return Err("Failed to log in!");
        }

        // We're logged in. Let's configure our headers.
        let csrf_token = response
            .headers()
            .get("X-CSRF-Token")
            .map(|value| value.to_str().unwrap_or(""))
            .unwrap_or("");
        let cookie = response
            .headers()
            .get("Set-Cookie")
            .map(|value| value.to_str().unwrap_or(""))
            .unwrap_or("");

        if !csrf_token.is_empty() {
            self.headers
                .insert("X-CSRF-Token", csrf_token.parse().unwrap());
        }

        // Save the refreshed cookie
        if !cookie.is_empty() {
            self.headers.insert("Cookie", cookie.parse().unwrap());
            return Ok(());
        }

        return Err("Failed to log in!");
    }

    async fn acquire_csrf_token(&mut self) -> bool {
        // We only need to acquire a token if we aren't already logged in, or we don't already have a token.
        if self.headers.contains_key("X-CSRF-Token") {
            return true;
        }

        // UniFi OS has cross-site request forgery protection built into its web management UI.
        // We use this fact to fingerprint it by connecting directly to the supplied Protect controller address
        // and see if there's a CSRF token waiting for us.
        //let client = Client::new();

        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(&self.uri)
            .send()
            .await
            .expect("Failed to make GET request to unifi protect controller");

        if response.status().is_success() {
            let csrf_token: Option<String> = response
                .headers()
                .get("X-CSRF-Token")
                .and_then(|value| value.to_str().ok().map(|s| String::from(s)));

            // We found a token.
            if !csrf_token.is_none() {
                self.headers
                    .insert("X-CSRF-Token", csrf_token.unwrap().parse().unwrap());
                return true;
            }
        }

        // Something went wrong, or no CSRF-Token is needed
        false
    }

    pub fn clear_login_credentials(&mut self) {
        self.headers.remove("Cookie");
    }
}
