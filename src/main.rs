use std::path::Path;
use parse_args::{Cli, Commands, DownloadCommand};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::Parser;
use unifi_protect::*;

mod parse_args;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Download(download_cmd) => download(download_cmd).await,
    };
}

async fn download(download_cmd: DownloadCommand) {
    let start_date = NaiveDateTime::parse_from_str(
        &(download_cmd.start_date + "-00:00:01"),
        "%Y-%m-%d-%H:%M:%S",
    )
    .expect("Failed to parse start date");
    let end_date = NaiveDateTime::parse_from_str(
        &(download_cmd.end_date + "-00:00:01"),
        "%Y-%m-%d-%H:%M:%S",
    )
    .expect("Failed to parse end date");

    let mut server = UnifiProtectServer::new(&download_cmd.host);
    println!("Logging in...");
    server
        .login(
            &*download_cmd.username,
            &*download_cmd.password,
        )
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
    if download_cmd.mode == "hourly" {
        for date in start_date.date().iter_days().take(
            end_date
                .date()
                .signed_duration_since(start_date.date())
                .num_days() as usize
                + 1,
        ) {
            for hour in 0..24 {
                let start_time = date.and_hms_opt(hour, 0, 0).expect("Failed to construct dateTime");
                let end_time = date.and_hms_opt(hour, 59, 59).expect("Failed to construct dateTime");
                time_frames.push((
                    Local.from_local_datetime(&start_time).unwrap(),
                    Local.from_local_datetime(&end_time).unwrap(),
                ));
            }
        }
    } else if download_cmd.mode == "daily" {
        for date in start_date.date().iter_days().take(
            end_date
                .date()
                .signed_duration_since(start_date.date())
                .num_days() as usize
                + 1,
        ) {
            let start_time = date.and_hms_opt(0, 0, 0).expect("Failed to construct dateTime");
            let end_time = date.and_hms_opt(23, 59, 59).expect("Failed to construct dateTime");
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
            let mut file_name = format!(
                "{}-{}-{}.mp4",
                time_frame.0.format("%Y-%m-%d-%H"),
                camera.name,
                download_cmd.recording_type
            );
            // sanitize filename using sanitize-filename and drop non-ascii symbols
            let options = sanitize_filename::Options {
                truncate: true, // true by default, truncates to 255 bytes
                windows: true, // default value depends on the OS, removes reserved names like `con` from start of strings on Windows
                replacement: "" // str to replace sanitized chars/strings
            };
            file_name = sanitize_filename::sanitize_with_options(file_name, options)
                .chars().filter(|s| s.is_ascii()).collect::<String>();

            let file_path = Path::new(&download_cmd.path)
                .join(file_name).as_os_str()
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
                download_cmd.recording_type,
                camera.name, file_path
            );
            if !server
                .download_footage(
                    camera,
                    &file_path,
                    &*download_cmd.recording_type,
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
