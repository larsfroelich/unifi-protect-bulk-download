pub mod feature_flags;
pub mod settings;
pub mod state;
pub mod stats;
pub mod util;

use feature_flags::*;
use settings::*;
use state::*;
use stats::*;
use util::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiProtectCameraSimple {
    pub id: String,
    pub mac: String,
    pub host: String,
    pub name: String,
    pub is_connected: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiProtectCamera {
    pub is_deleting: bool,
    pub mac: String,
    pub host: String,
    pub name: String,
    pub connection_host: String,
    pub connected_since: Option<i64>,
    pub state: String,
    pub hardware_revision: String,
    pub firmware_version: String,
    pub latest_firmware_version: Option<String>,
    pub firmware_build: String,
    pub is_updating: bool,
    #[serde(alias = "isDownloadingFW")]
    pub is_downloading_fw: bool,
    pub fw_update_state: Option<String>,
    pub is_adopting: bool,
    pub is_adopted: bool,
    pub is_adopted_by_other: bool,
    pub is_provisioned: bool,
    pub is_rebooting: bool,
    pub is_ssh_enabled: bool,
    pub can_adopt: bool,
    pub is_attempting_to_connect: bool,
    pub guid: Option<String>,
    pub last_motion: i64,
    pub mic_volume: i32,
    pub is_mic_enabled: bool,
    pub is_recording: bool,
    pub is_wireless_uplink_enabled: bool,
    pub is_motion_detected: bool,
    pub is_smart_detected: bool,
    pub phy_rate: Option<f64>,
    pub hdr_mode: bool,
    pub video_mode: String,
    pub is_probing_for_wifi: bool,
    pub ap_mac: Option<String>,
    pub ap_rssi: Option<i32>,
    pub ap_mgmt_ip: Option<String>,
    pub element_info: Option<String>,
    pub chime_duration: i32,
    pub is_dark: bool,
    pub last_privacy_zone_position_id: Option<i64>,
    pub last_ring: Option<i64>,
    pub is_live_heatmap_enabled: bool,
    pub anonymous_device_id: String,
    pub event_stats: EventStats,
    pub video_reconfiguration_in_progress: bool,
    pub voltage: Option<i32>,
    pub use_global: bool,
    pub is_poor_network: bool,
    pub stop_stream_level: Option<i32>,
    pub is_waterproof_case_attached: bool,
    pub last_disconnect: i64,
    pub user_configured_ap: bool,
    pub wired_connection_state: WiredConnectionState,
    pub channels: Vec<Channel>,
    pub isp_settings: IspSettings,
    pub talkback_settings: TalkbackSettings,
    pub osd_settings: OsdSettings,
    pub led_settings: LedSettings,
    pub speaker_settings: SpeakerSettings,
    pub recording_settings: RecordingSettings,
    pub smart_detect_settings: SmartDetectSettings,
    pub recording_schedules: Option<Vec<RecordingSchedule>>,
    pub recording_schedules_v2: Option<Vec<RecordingSchedule>>,
    pub motion_zones: Vec<MotionZone>,
    pub privacy_zones: Vec<PrivacyZone>,
    pub smart_detect_zones: Vec<SmartDetectZone>,
    pub smart_detect_lines: Vec<SmartDetectLine>,
    pub stats: Stats,
    pub feature_flags: FeatureFlags,
    pub pir_settings: PIRSettings,
    pub lcd_message: HashMap<String, String>,
    pub wifi_connection_state: WifiConnectionState,
    pub lenses: Vec<String>,
    pub stream_sharing: StreamSharing,
    pub homekit_settings: HomekitSettings,
    pub id: String,
    pub nvr_mac: String,
    pub is_connected: bool,
    pub platform: String,
    pub has_speaker: bool,
    pub has_wifi: bool,
    pub audio_bitrate: i32,
    pub can_manage: bool,
    pub is_managed: bool,
    pub market_name: String,
    #[serde(alias = "is4K")]
    pub is4k: bool,
    #[serde(alias = "is2K")]
    pub is2k: bool,
    pub model_key: String,
}

#[cfg(test)]
mod tests {
    use crate::UnifiProtectCamera;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn main() {
        // Open the JSON file
        let mut file = File::open("./src/sample_data/cameras.json").expect("Failed to open file");
        // Read the file contents into a String
        let mut cameras_json = String::new();
        file.read_to_string(&mut cameras_json)
            .expect("Failed to read file");

        // Parse the JSON data into the UnifiProtectCamera pub struct using Serde
        let cameras: Vec<UnifiProtectCamera> = serde_json::from_str(&cameras_json).unwrap();

        // Print the UnifiProtectCamera pub struct
        println!("{:?}", cameras);
    }
}
