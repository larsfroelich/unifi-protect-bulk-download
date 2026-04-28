use crate::app_error::{api_error, to_local, AppError};
use crate::parse_args::{parse_args, Commands, DownloadArgs, DownloadMode};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, Timelike};
use std::path::{Path, PathBuf};
use unifi_protect::*;

mod app_error;
mod output_path;
mod parse_args;

#[tokio::main]
async fn main() {
    let args = parse_args();

    match args.command {
        Commands::Download(download_args) => {
            if let Err(error) = download(&download_args).await {
                eprintln!("Download failed: {}", error);
                std::process::exit(1);
            }
        }
    }
}

async fn download(args: &DownloadArgs) -> Result<(), AppError> {
    output_path::validate(Path::new(&args.out_path), args.probe_output_path)?;

    let start_date = parse_date_or_hour(&args.start_date, true)
        .map_err(|source| AppError::parse_date(&args.start_date, source))?;
    let end_date = parse_date_or_hour(&args.end_date, false)
        .map_err(|source| AppError::parse_date(&args.end_date, source))?;

    if end_date < start_date {
        return Err(AppError::InvalidDateRange {
            start: start_date,
            end: end_date,
        });
    }

    let cameras = args.cameras.clone();

    println!("Cameras to Download: {:?}", cameras);

    let mut server = UnifiProtectServer::new(&args.uri);
    println!("Logging in...");
    server
        .login(&args.username, &args.password)
        .await
        .map_err(|source| api_error(format!("failed to login to '{}'", args.uri), source))?;
    println!("Logged in!");
    println!("Fetching cameras...");
    server
        .fetch_cameras(false)
        .await
        .map_err(|source| api_error("failed to fetch cameras", source))?;

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
                .ok_or_else(|| AppError::DateConstruction {
                    context: format!("failed to build hour start from '{}'", cursor),
                })?;
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
                to_local(frame_start, "hourly frame start".to_string())?,
                to_local(frame_end, "hourly frame end".to_string())?,
            ));
            cursor = hour_start + Duration::hours(1);
        }
    } else if matches!(args.mode, DownloadMode::Daily) {
        let mut date = start_date.date();
        while date <= end_date.date() {
            let day_start =
                date.and_hms_opt(0, 0, 0)
                    .ok_or_else(|| AppError::DateConstruction {
                        context: format!("failed to build day start from '{}'", date),
                    })?;
            let day_end =
                date.and_hms_opt(23, 59, 59)
                    .ok_or_else(|| AppError::DateConstruction {
                        context: format!("failed to build day end from '{}'", date),
                    })?;

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
                to_local(frame_start, "daily frame start".to_string())?,
                to_local(frame_end, "daily frame end".to_string())?,
            ));
            date = date.succ_opt().ok_or_else(|| AppError::DateOverflow {
                context: format!("failed to calculate next day after '{}'", date),
            })?;
        }
    } else {
        return Err(AppError::InvalidMode {
            mode: format!("{:?}", args.mode),
        });
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

            let file_path: PathBuf = Path::new(&args.out_path).join(file_name);
            let file_path_display = file_path.display().to_string();
            let file_path_lossy = file_path.to_string_lossy().to_string();

            // check if file exists
            if file_path.exists() {
                println!("File '{}' already exists, skipping...", file_path_display);
                continue;
            }
            println!(
                "Downloading {} video for camera '{}' (file path: {})",
                args.recording_type.as_str(),
                camera.name,
                file_path_display
            );
            if !server
                .download_footage(
                    camera,
                    &file_path_lossy,
                    args.recording_type.as_str(),
                    time_frame.0.timestamp_millis(),
                    time_frame.1.timestamp_millis(),
                )
                .await
                .map_err(|source| {
                    api_error(
                        format!(
                            "failed to download {} video for camera '{}' ({}) for timeframe '{}' to '{}' into '{}'",
                            args.recording_type.as_str(),
                            camera.name,
                            camera.id,
                            time_frame.0,
                            time_frame.1,
                            file_path_display
                        ),
                        source,
                    )
                })?
            {
                println!(
                    "No video found for time frame '{}' to '{}' for camera '{}'",
                    time_frame.0, time_frame.1, camera.name
                );
            }
        }
    }
    Ok(())
}

fn parse_date_or_hour(
    date_or_hour: &str,
    is_start: bool,
) -> Result<NaiveDateTime, chrono::ParseError> {
    // try to parse as date-time (YYYY-MM-DD-HH)
    if let Ok(date_time) =
        NaiveDateTime::parse_from_str(&format!("{}-00", date_or_hour), "%Y-%m-%d-%H-%M")
    {
        return Ok(date_time);
    }

    // hourly parsing failed, try to parse as date (YYYY-MM-DD)
    let date = NaiveDate::parse_from_str(date_or_hour, "%Y-%m-%d")?;
    Ok(if is_start {
        date.and_hms_opt(0, 0, 0)
    } else {
        date.and_hms_opt(23, 59, 59)
    }
    .expect("hard-coded time should be valid"))
}
