use std::{fs::OpenOptions, io::Write};

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::HeaderMap;
use reqwest::Client;

use super::cloudfront;
use crate::functions::logger::logger::{log, LogLevel};

pub async fn download_zip(
    cc: &str,
    index: usize,
    version: &u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let cc = if cc == "jp" {
        "battlecats"
    } else {
        &format!("battlecats{}", cc)
    };

    let version_fmt = if *version < 1000000 {
        format!("{}_{}_{}", cc, version, index)
    } else {
        format!(
            "{}_{:06}_{:02}_{:02}",
            cc,
            version / 100,
            index,
            version % 100
        )
    };

    let cloudfront = cloudfront::CloudFrontSign::new();

    let sign = cloudfront
        .generate_signed_cookie("https://nyanko-assets.ponosgames.com/*")
        .map_err(|e| {
            log(
                LogLevel::Error,
                format!("Error generating signed cookie: {}", e),
            );
            e
        })?;

    let mut headers = HeaderMap::new();
    headers.insert("accept-encoding", "gzip".parse()?);
    headers.insert("connection", "keep-alive".parse()?);
    headers.insert("cookie", sign.parse()?);
    headers.insert("range", "bytes=0-".parse()?);
    headers.insert(
        "user-agent",
        "Dalvik/2.1.0 (Linux; U; Android 13; XQ-BC52 Build/61.2.A.0.447)".parse()?,
    );

    let url = format!(
        "https://nyanko-assets.ponosgames.com/iphone/{}/download/{}.zip",
        cc, version_fmt
    );

    log(LogLevel::Info, format!("Downloading zip: {}", version_fmt));

    let client = Client::new();
    let response = client.get(&url).headers(headers).send().await?;

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
    pb.set_message(format!("Downloading: {}", version_fmt));

    let temp_dir_path = std::env::temp_dir().join("temp.zip");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temp_dir_path)?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download completed!");

    file.flush()?;

    log(
        LogLevel::Info,
        format!("Download completed: {}", version_fmt),
    );

    Ok(())
}
