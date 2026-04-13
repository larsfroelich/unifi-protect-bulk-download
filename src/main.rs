use crate::parse_args::{parse_args, Commands, DownloadArgs, DownloadMode};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike};
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
            let hour_start = cursor
                .date()
                .and_hms_opt(cursor.time().hour(), 0, 0)
                .expect("Failed to construct dateTime");
            let hour_end = hour_start + Duration::hours(1) - Duration::seconds(1);

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

            time_frames.push((
                Local.from_local_datetime(&frame_start).unwrap(),
                Local.from_local_datetime(&frame_end).unwrap(),
            ));
            cursor = hour_start + Duration::hours(1);
        }
    } else if matches!(args.mode, DownloadMode::Daily) {
        let mut date = start_date.date();
        while date <= end_date.date() {
            let day_start = date
                .and_hms_opt(0, 0, 0)
                .expect("Failed to construct dateTime");
            let day_end = date
                .and_hms_opt(23, 59, 59)
                .expect("Failed to construct dateTime");

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

            time_frames.push((
                Local.from_local_datetime(&frame_start).unwrap(),
                Local.from_local_datetime(&frame_end).unwrap(),
            ));
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
    if let Ok(date_time) = NaiveDateTime::parse_from_str(date_or_hour, "%Y-%m-%d-%H") {
        return Ok(date_time);
    }

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
