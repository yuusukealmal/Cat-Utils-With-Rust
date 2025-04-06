use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::config::structs::ServerAPK;
use crate::functions::logger::logger::{log, LogLevel};

pub mod version_details {
    pub fn get_address(data: &Vec<u8>, pattern: &[u32], start: Option<usize>) -> Option<usize> {
        let pattern_bytes: Vec<u8> = pattern.iter().flat_map(|num| num.to_le_bytes()).collect();

        match start {
            Some(start) => data[start..]
                .windows(pattern_bytes.len())
                .position(|window| window == &pattern_bytes)
                .map(|pos| pos + start),
            None => data
                .windows(pattern_bytes.len())
                .position(|window| window == &pattern_bytes),
        }
    }

    pub fn get_versions(data: &Vec<u8>, start: Option<usize>, end: Option<usize>) -> Vec<u32> {
        let mut versions = Vec::new();

        let start_address = start.unwrap_or(0);
        let end_address = end.unwrap_or(data.len());

        let mut i = start_address;
        while i + 4 <= end_address {
            if let Ok(bytes) = data[i..i + 4].try_into() {
                versions.push(u32::from_le_bytes(bytes));
            }
            i += 4;
        }

        versions
    }
}

impl ServerAPK {
    pub fn get_start_bytes_by_cc(&self) -> Option<Vec<u32>> {
        match self.cc.as_str() {
            "jp" => Some(vec![5, 5, 5, 7000000]),
            "en" => Some(vec![3, 2, 2, 6100000]),
            "kr" => Some(vec![3, 2, 1, 6100000]),
            "tw" => Some(vec![2, 3, 1, 6100000]),
            _ => None,
        }
    }

    pub fn get_version(&self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let file = File::open(std::env::temp_dir().join("lib.so"))?;

        let mut file_data = Vec::new();

        BufReader::new(file).read_to_end(&mut file_data)?;

        let start_versions = self.get_start_bytes_by_cc().unwrap();
        let start_address = version_details::get_address(&file_data, &start_versions, None);

        let end_address_1 = version_details::get_address(&file_data, &[0xFFFFFFFF], start_address);
        let end_address_2 = version_details::get_address(&file_data, &[0, 0, 0, 0], start_address);

        let end_address = match (end_address_1, end_address_2) {
            (Some(end_address_1), Some(end_address_2)) => min(end_address_1, end_address_2),
            (Some(end_address), None) => end_address,
            (None, Some(end_address)) => end_address,
            (None, None) => {
                log(LogLevel::Error, "Failed to find address".to_string());
                return Err("Address not found".into());
            }
        };

        let versions = version_details::get_versions(&file_data, start_address, Some(end_address));

        Ok(versions)
    }
}
