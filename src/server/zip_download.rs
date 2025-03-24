use futures::StreamExt;
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use reqwest::header::HeaderMap;

use crate::functions::logger::logger::{log, LogLevel};

use super::cloudfront;

pub async fn download_zip(cc: &str, index: usize, version: &u32) -> Result<(), Box<dyn std::error::Error>> {
    let cc = match cc {
        "jp" => "battlecats".to_string(),
        _ => format!("battlecats{}", cc),
    };

    let version_fmt = if version < &1000000 {
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

    let sign = match cloudfront
        .generate_signed_cookie("https://nyanko-assets.ponosgames.com/*") {
            Ok(sign) => sign,
            Err(e) => {
                println!("Error: {}", e);
                return Err(e.into());
            }
        };
    println!("{}", sign);
    let mut headers = HeaderMap::new();
    headers.insert("accept-encoding", "gzip".parse()?);
    headers.insert("connection", "keep-alive".parse()?);
    headers.insert("cookie", sign.parse()?);
    headers.insert("range", "bytes=0-".parse()?);
    headers.insert(
        "user-agent",
        "Dalvik/2.1.0 (Linux; U; Android 13; XQ-BC52 Build/61.2.A.0.447)"
            .parse()?,
    );

    let url = format!(
        "https://nyanko-assets.ponosgames.com/iphone/{}/download/{}.zip",
        cc, version_fmt
    );

    log(LogLevel::Info, format!("Downloading zip {}", version_fmt));

    let client = Client::new();
    let response = client.get(&url)
        .headers(headers)
        .send()
        .await?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}/temp.zip", std::env::temp_dir().display()))?;
    
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }
    
    Ok(())
}