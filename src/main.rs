// main.rs - Unifi Protect Bulk Download Tool
// Downloads footage from Unifi Protect NVR in bulk, with optional transcoding.
//
// Usage: unifi_protect_bulk_download download <uri> <username> <password> <path> <mode> <start_date> <end_date>
//
// Rewritten in 2026 to use the current Unifi Protect API (the old unifi_protect crate was outdated).

use std::path::Path;
use crate::parse_args::parse_args;
use crate::api::UnifiProtectClient;
use chrono::{DateTime, Local, TimeZone, NaiveDate};
use clap::ArgMatches;

mod parse_args;
mod api;

/// Type alias for a time range (start, end) in local time
type TimeFrame = (DateTime<Local>, DateTime<Local>);

#[tokio::main]
async fn main() {
    let args = parse_args();

    match args.subcommand() {
        Some(("download", args)) => {
            if let Err(e) = download(args).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            println!("No subcommand found! Use 'download' to download footage.");
            println!("Run with --help for usage information.");
        }
    }
}

/// Main download function - authenticates, lists cameras, and downloads footage
async fn download(args: &ArgMatches) -> Result<(), String> {
    // Parse the date range from arguments
    let start_date = NaiveDate::parse_from_str(
        args.get_one::<String>("start_date").unwrap(),
        "%Y-%m-%d",
    ).map_err(|e| format!("Failed to parse start date: {}", e))?;

    let end_date = NaiveDate::parse_from_str(
        args.get_one::<String>("end_date").unwrap(),
        "%Y-%m-%d",
    ).map_err(|e| format!("Failed to parse end date: {}", e))?;

    let uri = args.get_one::<String>("uri").unwrap();
    let username = args.get_one::<String>("username").unwrap();
    let password = args.get_one::<String>("password").unwrap();
    let out_path = args.get_one::<String>("out_path").unwrap();
    let mode = args.get_one::<String>("mode").unwrap();

    // Ensure output directory exists
    if !Path::new(out_path).exists() {
        std::fs::create_dir_all(out_path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Create the API client and authenticate
    let client = UnifiProtectClient::new(uri);

    println!("Logging in to {}...", uri);
    client.login(username, password).await?;
    println!("Logged in successfully!");

    // Fetch cameras
    println!("Fetching cameras...");
    let cameras = client.get_cameras().await?;
    println!("Found {} cameras:", cameras.len());

    for camera in &cameras {
        let status = if camera.is_connected { "online" } else { "offline" };
        println!("  [{}] {} - {} ({})", status, camera.name, camera.id, camera.mac);
    }

    // Calculate time frames based on mode
    let time_frames = calculate_time_frames(&start_date, &end_date, mode)?;
    println!("\nWill download {} time segments per camera", time_frames.len());

    // Download videos for each time frame and camera
    println!("\nDownloading videos...");
    let mut downloaded = 0;
    let mut skipped = 0;
    let mut no_recording = 0;

    for (start_time, end_time) in &time_frames {
        println!("\nTime frame: {} to {}",
            start_time.format("%Y-%m-%d %H:%M"),
            end_time.format("%Y-%m-%d %H:%M"));

        for camera in &cameras {
            // Build output filename
            let file_name = build_filename(&camera.name, start_time, mode);
            let file_path = Path::new(out_path).join(&file_name);
            let file_path_str = file_path.to_string_lossy().to_string();

            // Skip if file already exists (allows resuming)
            if file_path.exists() {
                println!("  [SKIP] {} (already exists)", file_name);
                skipped += 1;
                continue;
            }

            // Convert times to milliseconds for API
            let start_ms = start_time.timestamp_millis();
            let end_ms = end_time.timestamp_millis();

            // Build display filename for Unifi (shown in their UI)
            let display_filename = format!(
                "{} {} - {}.mp4",
                camera.name,
                format_time_for_api(start_time),
                format_time_for_api(end_time)
            );

            print!("  [....] {} - {}...", camera.name, file_name);

            match client.download_video(camera, start_ms, end_ms, &display_filename, &file_path_str).await {
                Ok(true) => {
                    println!("\r  [ OK ] {} - {}", camera.name, file_name);
                    downloaded += 1;
                }
                Ok(false) => {
                    println!("\r  [NONE] {} - {} (no recording)", camera.name, file_name);
                    no_recording += 1;
                }
                Err(e) => {
                    println!("\r  [FAIL] {} - {}: {}", camera.name, file_name, e);
                }
            }
        }
    }

    // Summary
    println!("\n========================================");
    println!("Download complete!");
    println!("  Downloaded: {}", downloaded);
    println!("  Skipped:    {} (already existed)", skipped);
    println!("  No recording: {}", no_recording);
    println!("========================================");

    Ok(())
}

/// Calculate time frames based on mode (hourly or daily)
fn calculate_time_frames(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    mode: &str,
) -> Result<Vec<TimeFrame>, String> {
    let mut time_frames = Vec::new();

    let num_days = end_date.signed_duration_since(*start_date).num_days() + 1;
    if num_days < 1 {
        return Err("End date must be on or after start date".to_string());
    }

    for day_offset in 0..num_days {
        let date = *start_date + chrono::Duration::days(day_offset);

        if mode == "hourly" {
            // One segment per hour
            for hour in 0..24 {
                let start_time = date.and_hms_opt(hour, 0, 0)
                    .ok_or("Failed to construct start time")?;
                let end_time = date.and_hms_opt(hour, 59, 59)
                    .ok_or("Failed to construct end time")?;

                time_frames.push((
                    Local.from_local_datetime(&start_time).single()
                        .ok_or("Failed to convert start time to local")?,
                    Local.from_local_datetime(&end_time).single()
                        .ok_or("Failed to convert end time to local")?,
                ));
            }
        } else {
            // Daily mode - one segment per day
            let start_time = date.and_hms_opt(0, 0, 0)
                .ok_or("Failed to construct start time")?;
            let end_time = date.and_hms_opt(23, 59, 59)
                .ok_or("Failed to construct end time")?;

            time_frames.push((
                Local.from_local_datetime(&start_time).single()
                    .ok_or("Failed to convert start time to local")?,
                Local.from_local_datetime(&end_time).single()
                    .ok_or("Failed to convert end time to local")?,
            ));
        }
    }

    Ok(time_frames)
}

/// Build output filename from camera name and time
fn build_filename(camera_name: &str, start_time: &DateTime<Local>, mode: &str) -> String {
    let time_part = if mode == "hourly" {
        start_time.format("%Y-%m-%d_%H00").to_string()
    } else {
        start_time.format("%Y-%m-%d").to_string()
    };

    // Sanitize the camera name for use in filename
    let options = sanitize_filename::Options {
        truncate: true,
        windows: true,
        replacement: "_",
    };

    let safe_name = sanitize_filename::sanitize_with_options(camera_name, options);
    format!("{}_{}.mp4", time_part, safe_name)
}

/// Format a datetime for the Unifi API filename parameter
/// Format: "M-DD-YYYY, H.MM.SSam/pm TZ"
fn format_time_for_api(dt: &DateTime<Local>) -> String {
    // Unifi expects format like: "1-15-2025, 8.00.00am EST"
    // Note: %Z gives the timezone abbreviation (EST, PST, etc.) not the offset
    let hour = dt.format("%I").to_string().trim_start_matches('0').to_string();
    let hour = if hour.is_empty() { "12".to_string() } else { hour };

    // Get timezone abbreviation - chrono's %Z should give EST/PST/etc.
    // But if it gives an offset, we need to map it to abbreviation
    let tz_str = dt.format("%Z").to_string();

    // If we got an offset like "-05:00", convert to abbreviation
    // Note: This is a simplified mapping. In January it's Standard Time, not Daylight.
    // The actual timezone depends on DST rules, but we'll use standard abbreviations.
    let tz_abbrev = match tz_str.as_str() {
        "-05:00" | "-0500" => "EST",  // Eastern Standard
        "-04:00" | "-0400" => "EDT",  // Eastern Daylight
        "-08:00" | "-0800" => "PST",  // Pacific Standard
        "-07:00" | "-0700" => "PDT",  // Pacific Daylight (or MST)
        "-06:00" | "-0600" => "CST",  // Central Standard
        other => other, // Use whatever chrono gave us if it's already an abbreviation
    };

    format!(
        "{}-{}-{}, {}.{}.{}{} {}",
        dt.format("%-m"),      // Month without leading zero
        dt.format("%d"),       // Day with leading zero
        dt.format("%Y"),       // 4-digit year
        hour,                  // Hour without leading zero
        dt.format("%M"),       // Minutes
        dt.format("%S"),       // Seconds
        dt.format("%P"),       // am/pm
        tz_abbrev,             // timezone abbreviation
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_filename() {
        let dt = Local.with_ymd_and_hms(2025, 1, 15, 14, 30, 0).unwrap();

        let hourly = build_filename("Front Door", &dt, "hourly");
        assert_eq!(hourly, "2025-01-15_1400_Front_Door.mp4");

        let daily = build_filename("Front Door", &dt, "daily");
        assert_eq!(daily, "2025-01-15_Front_Door.mp4");
    }

    #[test]
    fn test_sanitize_camera_name() {
        let dt = Local.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        let name = build_filename("Camera/With:Bad*Chars", &dt, "daily");
        assert!(!name.contains('/'));
        assert!(!name.contains(':'));
        assert!(!name.contains('*'));
    }
}
