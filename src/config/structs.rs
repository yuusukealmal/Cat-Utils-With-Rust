use std::{fs::File, path::PathBuf};

use zip::ZipArchive;

pub struct BCUZIP {
    pub title: String,
    pub length: u32,
    pub pad: u32,
    pub data: Vec<u8>,
    pub key: [u8; 16],
    pub iv: [u8; 16],
}

pub struct BCUFiles {
    pub offset: u32,
    pub path: String,
    pub size: u32,
}

pub struct Event {
    pub account_code: Option<String>,
    pub password: Option<String>,
    pub password_refresh_token: Option<String>,
    pub jwt_token: Option<String>,
    pub output_path: Option<PathBuf>,
}

pub struct APK {
    pub cc: String,
    pub output_path: String,
    pub zip: ZipArchive<File>,
}

#[allow(dead_code)]
pub struct LocalItem {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
    pub output_path: PathBuf,
}

pub struct Account {
    pub account: String,
    pub password: String,
    pub cc: String,
    pub version: u32,
}

pub struct SaveParser {
    pub address: usize,
    pub save_data: Vec<u8>,
}

pub struct ServerAPK {
    pub update: Option<bool>,
    pub cc: String,
    pub output_path: String,
    pub zip: ZipArchive<File>,
}

pub struct CloudFrontSign {
    pub(crate) cf_private_key: String,
    pub(crate) cf_key_pair_id: String,
}

#[allow(dead_code)]
pub struct ServerItem {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
    pub output_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct EventData {
    pub cc: Option<String>,
    pub id: u32,
    pub rare: u32,
    pub super_rare: u32,
    pub uber_rare: u32,
    pub legend: u32,
    pub banner_text: String,
    pub force: bool,
    pub gatya_data: Option<String>,
    pub unit_buy: Option<String>,
}
