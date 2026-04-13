use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiConnectionState {
    channel: Option<i32>,
    frequency: Option<i32>,
    phy_rate: Option<f32>,
    tx_rate: Option<f32>,
    signal_quality: Option<i32>,
    ssid: Option<String>,
    bssid: Option<String>,
    ap_name: Option<String>,
    experience: Option<String>,
    signal_strength: Option<i32>,
    connectivity: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSharing {
    enabled: bool,
    token: Option<String>,
    share_link: Option<String>,
    expires: Option<i64>,
    shared_by_user_id: Option<String>,
    shared_by_user: Option<String>,
    max_streams: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraFocus {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraPan {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraTilt {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraZoom {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hotplug {
    audio: Option<bool>,
    video: Option<bool>,
    extender: Extender,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extender {
    is_attached: bool,
    has_flash: Option<bool>,
    has_ir: Option<bool>,
    has_radar: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WiredConnectionState {
    phy_rate: Option<i32>,
}
