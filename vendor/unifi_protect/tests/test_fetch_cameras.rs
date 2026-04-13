#[cfg(test)]
mod tests {
    use unifi_protect::UnifiProtectServer;

    #[tokio::test]
    #[ignore]
    async fn fetch_cameras_test() {
        let mut server = UnifiProtectServer::new("BASE_URI"); // ( e.g. "https://192.168.1.28")
        server
            .login("USERNAME", "PASSWORD")
            .await
            .expect("Failed to login");
        server
            .fetch_cameras(true)
            .await
            .expect("Failed to fetch cameras");

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
}
