use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags {
    can_adjust_ir_led_level: bool,
    can_magic_zoom: bool,
    can_optical_zoom: bool,
    can_touch_focus: bool,
    has_accelerometer: bool,
    has_aec: bool,
    #[serde(alias = "hasAutoICROnly")]
    has_auto_icronly: bool,
    has_battery: bool,
    has_bluetooth: bool,
    has_chime: bool,
    has_external_ir: bool,
    has_icr_sensitivity: bool,
    has_infrared: bool,
    has_ldc: bool,
    has_led_ir: bool,
    has_led_status: bool,
    has_line_in: bool,
    has_mic: bool,
    has_privacy_mask: bool,
    has_rtc: bool,
    has_sd_card: bool,
    has_speaker: bool,
    has_wifi: bool,
    has_hdr: bool,
    video_modes: Vec<String>,
    video_mode_max_fps: Vec<i32>,
    has_motion_zones: bool,
    has_lcd_screen: bool,
    mount_positions: Vec<String>,
    smart_detect_types: Vec<String>,
    smart_detect_audio_types: Vec<String>,
    lens_type: Option<String>,
    lens_model: Option<String>,
    motion_algorithms: Vec<String>,
    has_square_event_thumbnail: bool,
    has_package_camera: bool,
    audio: Vec<String>,
    audio_codecs: Vec<String>,
    is_doorbell: bool,
    privacy_mask_capability: PrivacyMaskCapability,
    focus: FocusSettings,
    pan: PanSettings,
    tilt: TiltSettings,
    zoom: ZoomSettings,
    hotplug: HotplugSettings,
    has_smart_detect: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyMaskCapability {
    max_masks: i32,
    rectangle_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSettings {
    steps: FocusSteps,
    degrees: FocusDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanSettings {
    steps: PanSteps,
    degrees: PanDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltSettings {
    steps: TiltSteps,
    degrees: TiltDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomSettings {
    steps: ZoomSteps,
    degrees: ZoomDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HotplugSettings {
    audio: Option<bool>,
    video: Option<bool>,
    extender: Option<ExtenderSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtenderSettings {
    is_attached: bool,
    has_flash: Option<bool>,
    has_ir: Option<bool>,
    has_radar: Option<bool>,
}
