use crate::parse_args::{parse_args, Commands, DownloadArgs, DownloadMode};
use chrono::{
    DateTime, Duration, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone, Timelike,
};
use std::path::Path;
use unifi_protect::*;

mod parse_args;

#[tokio::main]
async fn main() {
    let args = parse_args();

    match args.command {
        Commands::Download(download_args) => {
            download(&download_args).await;
        }
    }
}

async fn download(args: &DownloadArgs) {
    let start_date =
        parse_date_or_hour(&args.start_date, true).expect("Failed to parse start date");
    let end_date = parse_date_or_hour(&args.end_date, false).expect("Failed to parse end date");

    if end_date < start_date {
        panic!("Invalid date range: end date/time is before start date/time");
    }

    let cameras = args.cameras.clone();

    println!("Cameras to Download: {:?}", cameras);

    let mut server = UnifiProtectServer::new(&args.uri);
    println!("Logging in...");
    server
        .login(&args.username, &args.password)
        .await
        .expect("Failed to login");
    println!("Logged in!");
    println!("Fetching cameras...");
    server
        .fetch_cameras(false)
        .await
        .expect("Failed to fetch cameras");

    println!("Found {} cameras", server.cameras_simple.len());
    for camera in server.cameras_simple.iter() {
        println!(
            "Camera: {} {} {} '{}'",
            (if camera.is_connected {
                "<online>"
            } else {
                "<offline>"
            }),
            &camera.mac,
            &camera.id,
            &camera.name
        );
    }

    // Calculate time frames
    let mut time_frames: Vec<(DateTime<Local>, DateTime<Local>)> = vec![];
    if matches!(args.mode, DownloadMode::Hourly) {
        let mut cursor = start_date;
        while cursor <= end_date {
            let (hour_start, hour_end) = match hour_frame_bounds(cursor) {
                Ok(bounds) => bounds,
                Err(err) => {
                    println!("Skipping hour frame for '{}': {}", cursor, err);
                    cursor = cursor
                        .date()
                        .and_hms_opt(cursor.time().hour(), 0, 0)
                        .expect("Failed to construct dateTime")
                        + Duration::hours(1);
                    continue;
                }
            };

            let frame_start = if hour_start < start_date {
                start_date
            } else {
                hour_start
            };
            let frame_end = if hour_end > end_date {
                end_date
            } else {
                hour_end
            };

            // Resolve local boundaries with explicit DST handling so timestamps passed
            // to download_footage are deterministic across ambiguous transitions.
            let frame_start = resolve_local_datetime(&Local, frame_start, BoundaryKind::Start)
                .expect("Failed to resolve frame start in local timezone");
            let frame_end = resolve_local_datetime(&Local, frame_end, BoundaryKind::End)
                .expect("Failed to resolve frame end in local timezone");
            time_frames.push((frame_start, frame_end));
            cursor = hour_start + Duration::hours(1);
        }
    } else if matches!(args.mode, DownloadMode::Daily) {
        let mut date = start_date.date();
        while date <= end_date.date() {
            let (day_start, day_end) =
                day_frame_bounds(date).expect("Failed to construct local day frame bounds");

            let frame_start = if day_start < start_date {
                start_date
            } else {
                day_start
            };
            let frame_end = if day_end > end_date {
                end_date
            } else {
                day_end
            };

            // Resolve local boundaries with explicit DST handling so timestamps passed
            // to download_footage are deterministic across ambiguous transitions.
            let frame_start = resolve_local_datetime(&Local, frame_start, BoundaryKind::Start)
                .expect("Failed to resolve frame start in local timezone");
            let frame_end = resolve_local_datetime(&Local, frame_end, BoundaryKind::End)
                .expect("Failed to resolve frame end in local timezone");
            time_frames.push((frame_start, frame_end));
            date = date.succ_opt().expect("Failed to calculate next date");
        }
    } else {
        println!("Invalid mode!");
    }

    println!("Downloading videos...");
    for time_frame in time_frames {
        println!(
            "Downloading video for time frame '{}' to '{}'",
            time_frame.0, time_frame.1
        );
        for camera in server.cameras_simple.iter() {
            // check if camera name or id is in the list of cameras to download but if list contains all or * then download all
            if !cameras.contains(&camera.name)
                && !cameras.contains(&camera.id)
                && !cameras.contains(&"*".to_string())
                && !cameras.contains(&"all".to_string())
            {
                continue;
            }
            let mut file_name = format!(
                "{}-{}-{}.mp4",
                time_frame.0.format("%Y-%m-%d-%H"),
                camera.name,
                args.recording_type.as_str()
            );
            // sanitize filename using sanitize-filename and drop non-ascii symbols
            let options = sanitize_filename::Options {
                truncate: true,  // true by default, truncates to 255 bytes
                windows: true, // default value depends on the OS, removes reserved names like `con` from start of strings on Windows
                replacement: "", // str to replace sanitized chars/strings
            };
            file_name = sanitize_filename::sanitize_with_options(file_name, options)
                .chars()
                .filter(|s| s.is_ascii())
                .collect::<String>();

            let file_path = Path::new(&args.out_path)
                .join(file_name)
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string();

            // check if file exists
            if Path::new(&file_path).exists() {
                println!("File '{}' already exists, skipping...", file_path);
                continue;
            }
            println!(
                "Downloading {} video for camera '{}' (file path: {})",
                args.recording_type.as_str(),
                camera.name,
                file_path
            );
            if !server
                .download_footage(
                    camera,
                    &file_path,
                    args.recording_type.as_str(),
                    time_frame.0.timestamp_millis(),
                    time_frame.1.timestamp_millis(),
                )
                .await
                .expect("Failed to download video")
            {
                println!(
                    "No video found for time frame '{}' to '{}' for camera '{}'",
                    time_frame.0, time_frame.1, camera.name
                );
            }
        }
    }
    return;
}

fn parse_date_or_hour(
    date_or_hour: &str,
    is_start: bool,
) -> Result<NaiveDateTime, chrono::ParseError> {
    // try to parse as date-time (YYYY-MM-DD-HH)
    if let Ok(date_time) = NaiveDateTime::parse_from_str(date_or_hour, "%Y-%m-%d-%H") {
        return Ok(date_time);
    }

    // hourly parsing failed, try to parse as date (YYYY-MM-DD)
    let date = NaiveDate::parse_from_str(date_or_hour, "%Y-%m-%d")?;
    if is_start {
        Ok(date
            .and_hms_opt(0, 0, 0)
            .expect("Failed to construct dateTime"))
    } else {
        Ok(date
            .and_hms_opt(23, 59, 59)
            .expect("Failed to construct dateTime"))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BoundaryKind {
    // For duplicated local times during fall DST transition, select the earliest
    // instant for a frame start so we don't miss data at the beginning.
    Start,
    // For duplicated local times during fall DST transition, select the latest
    // instant for a frame end so frame windows remain inclusive.
    End,
}

// Convert a naive local timestamp into a timezone-aware timestamp with explicit
// DST behavior:
// - Single: use the only valid instant.
// - Ambiguous: pick earliest for starts, latest for ends (deterministic).
// - None: return an error for skipped local timestamps (spring DST transition).
fn resolve_local_datetime<Tz: TimeZone>(
    tz: &Tz,
    local_datetime: NaiveDateTime,
    boundary_kind: BoundaryKind,
) -> Result<DateTime<Tz>, String> {
    match tz.from_local_datetime(&local_datetime) {
        LocalResult::Single(datetime) => Ok(datetime),
        LocalResult::Ambiguous(earliest, latest) => Ok(match boundary_kind {
            BoundaryKind::Start => earliest,
            BoundaryKind::End => latest,
        }),
        LocalResult::None => Err(format!(
            "Local datetime '{}' does not exist in the target timezone",
            local_datetime
        )),
    }
}

// Build naive day boundaries before timezone conversion.
fn day_frame_bounds(date: NaiveDate) -> Result<(NaiveDateTime, NaiveDateTime), String> {
    let day_start = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| format!("Failed to construct start of day for '{}'", date))?;
    let day_end = date
        .and_hms_opt(23, 59, 59)
        .ok_or_else(|| format!("Failed to construct end of day for '{}'", date))?;
    Ok((day_start, day_end))
}

// Build naive hour boundaries before timezone conversion.
fn hour_frame_bounds(date_time: NaiveDateTime) -> Result<(NaiveDateTime, NaiveDateTime), String> {
    let hour_start = date_time
        .date()
        .and_hms_opt(date_time.time().hour(), 0, 0)
        .ok_or_else(|| format!("Failed to construct start of hour for '{}'", date_time))?;
    let hour_end = hour_start + Duration::hours(1) - Duration::seconds(1);
    Ok((hour_start, hour_end))
}

#[cfg(test)]
mod tests {
    use super::{resolve_local_datetime, BoundaryKind};
    use chrono::{NaiveDate, TimeZone, Utc};
    use chrono_tz::America::New_York;

    #[test]
    fn resolves_ambiguous_hour_deterministically() {
        let ambiguous = New_York
            .with_ymd_and_hms(2025, 11, 2, 1, 30, 0)
            .earliest()
            .expect("Expected local time to be ambiguous")
            .naive_local();

        let start_dt = resolve_local_datetime(&New_York, ambiguous, BoundaryKind::Start)
            .expect("Start boundary resolution should succeed");
        let end_dt = resolve_local_datetime(&New_York, ambiguous, BoundaryKind::End)
            .expect("End boundary resolution should succeed");

        assert_eq!(
            start_dt.with_timezone(&Utc).timestamp(),
            1_762_061_400,
            "Start boundary should consistently choose earliest UTC instant"
        );
        assert_eq!(
            end_dt.with_timezone(&Utc).timestamp(),
            1_762_065_000,
            "End boundary should consistently choose latest UTC instant"
        );
    }

    #[test]
    fn returns_error_for_skipped_hour() {
        let skipped = NaiveDate::from_ymd_opt(2025, 3, 9)
            .expect("Expected valid date")
            .and_hms_opt(2, 30, 0)
            .expect("Expected valid naive local timestamp");
        assert!(
            New_York
                .with_ymd_and_hms(2025, 3, 9, 2, 30, 0)
                .single()
                .is_none(),
            "Expected skipped hour in spring DST transition"
        );

        let result = resolve_local_datetime(&New_York, skipped, BoundaryKind::Start);
        assert!(
            result.is_err(),
            "Skipped local timestamp must return an error"
        );
    }
}
