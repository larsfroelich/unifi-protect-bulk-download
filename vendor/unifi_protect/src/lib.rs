pub mod api;
pub mod auth;
pub mod camera;

use camera::*;
use reqwest::header::HeaderMap;

pub struct UnifiProtectServer {
    pub uri: String,
    pub cameras: Vec<UnifiProtectCamera>,
    pub cameras_simple:Vec<UnifiProtectCameraSimple>,
    headers: HeaderMap,
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    error: String,
}

impl UnifiProtectServer {
    pub fn new(uri: &str) -> UnifiProtectServer {
        UnifiProtectServer {
            uri: if uri.ends_with("/") { String::from(uri.split_at(uri.len()-1).0)} else { uri.to_string() },
            cameras: Vec::new(),
            cameras_simple: Vec::new(),
            headers: Default::default(),
        }
    }
}
