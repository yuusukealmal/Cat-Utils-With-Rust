use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use crate::functions::logger::logger::{log, LogLevel};

pub async fn download_apk(cc: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data_json: serde_json::Value = serde_json::from_str(&fs::read_to_string("data.json")?)?;
    let url = data_json[cc]["download_url"].as_str().unwrap();

    log(LogLevel::Info, format!("Downloading XAPK for cc: {}", cc));

    let client = Client::new();
    let response = client.get(url).send().await?;

    let total_size = response.content_length().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unable to get content length",
        )
    })?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("{msg} [{bar:40}] {bytes}/{total_bytes} ({percent}%)")
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_message(format!("Downloading XAPK for cc: {}", cc));

    let temp_dir_path = std::env::temp_dir().join("temp.xapk");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temp_dir_path)?;
    let mut writer = BufWriter::new(file);

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        writer.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        pb.set_position(downloaded);
    }

    writer.flush()?;
    pb.finish_with_message("Download completed!");

    Ok(())
}
