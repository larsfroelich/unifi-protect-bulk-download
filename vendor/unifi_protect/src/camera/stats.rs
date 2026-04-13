use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    rx_bytes: i64,
    tx_bytes: i64,
    wifi: WifiStats,
    battery: BatteryStats,
    video: VideoStats,
    storage: StorageStats,
    wifi_quality: i32,
    wifi_strength: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventStats {
    motion: MotionStats,
    smart: SmartStats,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionStats {
    today: i32,
    average: i32,
    last_days: Vec<i32>,
    recent_hours: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartStats {
    today: i32,
    average: i32,
    last_days: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiStats {
    channel: i32,
    frequency: i32,
    link_speed_mbps: Option<f32>,
    signal_quality: i32,
    signal_strength: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryStats {
    percentage: Option<i32>,
    is_charging: bool,
    sleep_state: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStats {
    recording_start: i64,
    recording_end: i64,
    #[serde(alias = "recordingStartLQ")]
    recording_start_lq: i64,
    #[serde(alias = "recordingEndLQ")]
    recording_end_lq: i64,
    timelapse_start: i64,
    timelapse_end: i64,
    #[serde(alias = "timelapseStartLQ")]
    timelapse_start_lq: i64,
    #[serde(alias = "timelapseEndLQ")]
    timelapse_end_lq: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStats {
    used: Option<i64>,
    rate: Option<f64>,
}
