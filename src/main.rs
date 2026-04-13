use crate::parse_args::{parse_args, Commands, DownloadArgs, DownloadMode};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
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
    let start_date = NaiveDateTime::parse_from_str(
        &(args.start_date.clone() + "-00:00:01"),
        "%Y-%m-%d-%H:%M:%S",
    )
    .expect("Failed to parse start date");
    let end_date =
        NaiveDateTime::parse_from_str(&(args.end_date.clone() + "-00:00:01"), "%Y-%m-%d-%H:%M:%S")
            .expect("Failed to parse end date");

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
        for date in start_date.date().iter_days().take(
            end_date
                .date()
                .signed_duration_since(start_date.date())
                .num_days() as usize
                + 1,
        ) {
            for hour in 0..24 {
                let start_time = date
                    .and_hms_opt(hour, 0, 0)
                    .expect("Failed to construct dateTime");
                let end_time = date
                    .and_hms_opt(hour, 59, 59)
                    .expect("Failed to construct dateTime");
                time_frames.push((
                    Local.from_local_datetime(&start_time).unwrap(),
                    Local.from_local_datetime(&end_time).unwrap(),
                ));
            }
        }
    } else if matches!(args.mode, DownloadMode::Daily) {
        for date in start_date.date().iter_days().take(
            end_date
                .date()
                .signed_duration_since(start_date.date())
                .num_days() as usize
                + 1,
        ) {
            let start_time = date
                .and_hms_opt(0, 0, 0)
                .expect("Failed to construct dateTime");
            let end_time = date
                .and_hms_opt(23, 59, 59)
                .expect("Failed to construct dateTime");
            time_frames.push((
                Local.from_local_datetime(&start_time).unwrap(),
                Local.from_local_datetime(&end_time).unwrap(),
            ));
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
