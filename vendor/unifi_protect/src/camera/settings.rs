use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IspSettings {
    ae_mode: String,
    ir_led_mode: String,
    ir_led_level: i32,
    wdr: i32,
    icr_sensitivity: i32,
    brightness: i32,
    contrast: i32,
    hue: i32,
    saturation: i32,
    sharpness: i32,
    denoise: i32,
    is_color_night_vision_enabled: bool,
    is_flipped_vertical: bool,
    is_flipped_horizontal: bool,
    is_auto_rotate_enabled: bool,
    is_ldc_enabled: bool,
    is3dnr_enabled: bool,
    is_external_ir_enabled: bool,
    is_aggressive_anti_flicker_enabled: bool,
    is_pause_motion_enabled: bool,
    d_zoom_center_x: i32,
    d_zoom_center_y: i32,
    d_zoom_scale: i32,
    d_zoom_stream_id: i32,
    focus_mode: String,
    focus_position: i32,
    touch_focus_x: i32,
    touch_focus_y: i32,
    zoom_position: i32,
    mount_position: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TalkbackSettings {
    type_fmt: String,
    type_in: String,
    bind_addr: String,
    bind_port: i32,
    filter_addr: Option<String>,
    filter_port: Option<i32>,
    channels: i32,
    sampling_rate: i32,
    bits_per_sample: i32,
    quality: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdSettings {
    is_name_enabled: bool,
    is_date_enabled: bool,
    is_logo_enabled: bool,
    is_debug_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LedSettings {
    is_enabled: bool,
    blink_rate: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerSettings {
    is_enabled: bool,
    are_system_sounds_enabled: bool,
    volume: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSettings {
    pre_padding_secs: i32,
    post_padding_secs: i32,
    min_motion_event_trigger: i32,
    end_motion_event_delay: i32,
    suppress_illumination_surge: bool,
    mode: String,
    geofencing: String,
    motion_algorithm: String,
    enable_motion_detection: bool,
    enable_pir_timelapse: bool,
    use_new_motion_algorithm: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PIRSettings {
    pir_sensitivity: i32,
    pir_motion_clip_length: i32,
    timelapse_frame_interval: i32,
    timelapse_transfer_interval: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomekitSettings {
    talkback_settings_active: bool,
    stream_in_progress: bool,
    microphone_muted: bool,
    speaker_muted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectSettings {
    object_types: Vec<String>,
    auto_tracking_object_types: Vec<String>,
    audio_types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordingSchedule {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MotionZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrivacyZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartDetectZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartDetectLine {
    // fields unknown
}
