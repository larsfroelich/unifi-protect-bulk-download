use std::path::Path;
use crate::parse_args::parse_args;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::ArgMatches;
use unifi_protect::*;

mod parse_args;

#[tokio::main]
async fn main() {
    let args = parse_args();
    println!("Args: {:?}!", args);

    match args.subcommand() {
        Some(("download", args)) => {
            download(args).await;
        }
        _ => {
            println!("No subcommand found!");
        }
    }
}

async fn download(args: &ArgMatches) {
    let start_date = NaiveDateTime::parse_from_str(
        &(String::from(args.get_one::<String>("start_date").unwrap()) + "-00:00:01"),
        "%Y-%m-%d-%H:%M:%S",
    )
    .expect("Failed to parse start date");
    let end_date = NaiveDateTime::parse_from_str(
        &(String::from(args.get_one::<String>("end_date").unwrap()) + "-00:00:01"),
        "%Y-%m-%d-%H:%M:%S",
    )
    .expect("Failed to parse end date");

    let mut server = UnifiProtectServer::new(args.get_one::<String>("uri").unwrap());
    println!("Logging in...");
    server
        .login(
            args.get_one::<String>("username").unwrap(),
            args.get_one::<String>("password").unwrap(),
        )
        .await
        .expect("Failed to login");
    println!("Logged in!");
    println!("Fetching cameras...");
    server
        .fetch_cameras()
        .await
        .expect("Failed to fetch cameras");

    println!("Found {} cameras", server.cameras.len());
    for camera in server.cameras.iter() {
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
    if args.get_one::<String>("mode").unwrap() == "hourly" {
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
    } else if args.get_one::<String>("mode").unwrap() == "daily" {
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
        for camera in server.cameras.iter() {
            let mut file_path = Path::new(&args.get_one::<String>("out_path").unwrap())
                .join(format!(
                    "{}-{}-{}.mp4",
                    time_frame.0.format("%Y-%m-%d-%H"),
                    camera.name,
                    &args.get_one::<String>("recording_type").unwrap()
                )).as_os_str()
                .to_str()
                .unwrap()
                .to_string();

            // sanitize filename using sanitize-filename
            let options = sanitize_filename::Options {
                truncate: true, // true by default, truncates to 255 bytes
                windows: true, // default value depends on the OS, removes reserved names like `con` from start of strings on Windows
                replacement: "" // str to replace sanitized chars/strings
            };
            file_path = sanitize_filename::sanitize_with_options(file_path, options);

            // check if file exists
            if Path::new(&file_path).exists() {
                println!("File '{}' already exists, skipping...", file_path);
                continue;
            }
            println!(
                "Downloading rotating video for camera '{}' (file path: {})",
                camera.name, file_path
            );
            if !server
                .download_footage(
                    camera,
                    &file_path,
                    args.get_one::<String>("recording_type").unwrap(),
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
