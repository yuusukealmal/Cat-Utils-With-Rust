use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::HeaderMap;
use reqwest::Client;

use crate::config::routes::{CLOUDFRONT_SIGN_URL, SERVER_ASSETS_ZIP};
use crate::config::structs::{CloudFrontSign, ServerAPK};
use crate::functions::logger::logger::{log, LogLevel};

impl ServerAPK {
    fn get_region_by_index(index: usize, region_counts: &Vec<i32>) -> Option<(String, usize)> {
        let region_order = vec!["", "fr", "it", "de", "es", "th"];
        let mut start = 0;
        for (i, &end) in region_counts.iter().enumerate() {
            if index < end as usize {
                return Some((region_order[i].to_string(), index - start));
            }
            start = end as usize;
        }
        None
    }

    pub async fn download_zip(
        &self,
        index: usize,
        version: &u32,
        tsvs: &(Vec<String>, Vec<i32>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cc = if &self.cc == "jp" {
            "battlecats"
        } else {
            &format!("battlecats{}", self.cc)
        };

        let (region, index) = match self.cc.as_str() {
            "en" => Self::get_region_by_index(index, &tsvs.1).unwrap(),
            _ => (self.cc.to_string(), index),
        };

        let mut version_fmt = if *version < 1000000 {
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

        if self.cc.as_str() == "en" && !region.is_empty() {
            version_fmt = format!("{}_{}", version_fmt, region);
        }

        let cloudfront = CloudFrontSign::new();

        let sign = cloudfront
            .generate_signed_cookie(CLOUDFRONT_SIGN_URL)
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

        let url = SERVER_ASSETS_ZIP(cc, &version_fmt);

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
}
