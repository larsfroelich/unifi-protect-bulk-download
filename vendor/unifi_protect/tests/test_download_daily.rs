#[cfg(test)]
mod tests {
    use unifi_protect::UnifiProtectServer;

    #[tokio::test]
    #[ignore]
    async fn download_daily() {
        let mut server = UnifiProtectServer::new("BASE_URI"); // ( e.g. "https://192.168.1.28")
        server
            .login("USERNAME", "PASSWORD")
            .await
            .expect("Failed to login");
        server
            .fetch_cameras(false)
            .await
            .expect("Failed to fetch cameras");
        println!("Found {} cameras", server.cameras_simple.len());
        println!(
            "Downloading rotating video for camera '{}'",
            server.cameras_simple[0].name
        );

        let camera = &server.cameras_simple[0];
        server
            .download_footage(
                camera,
                "/home/user/Desktop/video_test.mp4",
                "rotating",
                1678748401000,
                1678848401000,
            )
            .await
            .expect("Failed to download video");
    }
}
