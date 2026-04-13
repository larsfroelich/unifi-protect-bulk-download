use crate::camera::UnifiProtectCameraSimple;
use crate::{ErrorResponse, UnifiProtectServer};
use reqwest::Client;
use tokio::io::AsyncWriteExt;

impl UnifiProtectServer {
    pub async fn download_footage(
        &self,
        camera: &UnifiProtectCameraSimple,
        output_path: &str,
        recording_type: &str,
        start_unix: i64,
        end_unix: i64,
    ) -> Result<bool, String> {
        for channel in 0..4 {
            let endpoint = format!(
                "{}/proxy/protect/api/video/export?camera={}\
                {}\
                &channel={}\
                &filename={}.mp4\
                &lens=0\
                &start={}\
                &end={}\
                &type={}",
                self.uri,
                camera.id,
                (if recording_type == "timelapse" {
                    "&fps=4"
                } else {
                    ""
                }),
                channel,
                camera.mac,
                start_unix,
                end_unix,
                recording_type
            );

            let mut response = Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap()
                .get(&endpoint)
                .headers(self.headers.clone())
                .send()
                .await
                .expect("Failed to send request");

            if !response.status().is_success() {
                eprintln!("Error: {:?}", response);
                let status_code = response.status();
                let error_msg = response.json::<ErrorResponse>().await.ok().map(|x| x.error);
                if error_msg.is_some()
                    && (error_msg.as_ref().unwrap().contains("o files found")
                        || error_msg
                            .as_ref()
                            .unwrap()
                            .contains("track information is not valid"))
                {
                    continue;
                } else {
                    if error_msg.is_some() {
                        eprintln!("Error: {}", error_msg.unwrap());
                    } else {
                        eprintln!("Unknown Error - Status Code: {}", status_code);
                    }
                    eprintln!("Failed to download video.");
                    continue;
                }
            }

            let mut file = tokio::fs::File::create(output_path)
                .await
                .expect("Failed to create file");

            while let Some(chunk) = response.chunk().await.expect("Failed to read response chunk") {
                file.write_all(&chunk)
                    .await
                    .expect("Failed to write video chunk to file");
            }

            return Ok(true);
        }

        Ok(false)
    }
}
