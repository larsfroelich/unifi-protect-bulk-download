use clap::ArgMatches;
use crate::parse_args::{parse_args};
use unifi_protect::*;
mod parse_args;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args = parse_args();

    println!("Args: {:?}!", args);

    match args.subcommand() {
        Some(("download", args)) => {
            download(args).await;
        },
        _ => {
            println!("No subcommand found!");
        }
    }
}

async fn download(args : &ArgMatches){
    let mut server = UnifiProtectServer::new(args.get_one::<String>("uri").unwrap());
    println!("Logging in...");
    server.login(args.get_one::<String>("username").unwrap(), args.get_one::<String>("password").unwrap())
        .await.expect("Failed to login");
    println!("Logged in!");
    println!("Fetching cameras...");
    server.fetch_cameras()
        .await.expect("Failed to fetch cameras");

    println!("Found {} cameras", server.cameras.len());
    for camera in server.cameras.iter() {
        println!(
            "Camera: {} {} '{}'",
            (if camera.is_connected {
                "<online>"
            } else {
                "<offline>"
            }),
            &camera.mac,
            &camera.name
        );
    }
}