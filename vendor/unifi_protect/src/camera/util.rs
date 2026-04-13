use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: i32,
    video_id: String,
    name: String,
    enabled: bool,
    is_rtsp_enabled: bool,
    rtsp_alias: Option<String>,
    width: i32,
    height: i32,
    fps: i32,
    bitrate: i32,
    min_bitrate: i32,
    max_bitrate: i32,
    min_client_adaptive_bit_rate: i32,
    min_motion_adaptive_bit_rate: i32,
    fps_values: Vec<i32>,
    idr_interval: i32,
}
