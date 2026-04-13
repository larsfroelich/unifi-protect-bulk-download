#[cfg(test)]
mod tests {
    use unifi_protect::UnifiProtectServer;

    #[tokio::test]
    #[ignore]
    async fn login_test() {
        let mut server = UnifiProtectServer::new("BASE_URI"); // ( e.g. "https://192.168.1.28")
        server
            .login("USERNAME", "PASSWORD")
            .await
            .expect("Failed to login");
        println!("Logged in!");
    }
}
