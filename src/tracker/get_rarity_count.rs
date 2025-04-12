use std::fs;
use std::path::PathBuf;

use crate::config::routes::TRACK_UNIT_EXPLANATION;
use crate::config::structs::EventData;
use crate::functions::utils::get_folder_name;

impl EventData {
    pub async fn get_cat_name(&self, id: u32) -> Result<String, Box<dyn std::error::Error>> {
        let event_path = PathBuf::from(std::env::current_dir()?)
            .join("Data")
            .join(get_folder_name(&self.cc.as_ref().unwrap()))
            .join("Local")
            .join("resLocal")
            .join(format!("Unit_Explanation{}_tw.csv", id + 1));

        let explanations = match event_path.exists() && event_path.is_file() {
            true => fs::read_to_string(event_path)?,
            false => {
                reqwest::get(TRACK_UNIT_EXPLANATION(&self.cc.as_ref().unwrap(), id + 1))
                    .await?
                    .text()
                    .await?
            }
        };

        let name = explanations
            .lines()
            .nth(0)
            .unwrap()
            .split("|")
            .nth(0)
            .unwrap()
            .to_string();

        Ok(name)
    }

    pub async fn get_cat_ids(&self) -> Result<Vec<Vec<(i32, String)>>, Box<dyn std::error::Error>> {
        let mut cat_ids = vec![vec![]; 6];

        let rarities: Vec<u32> = self
            .unit_buy
            .as_ref()
            .unwrap()
            .lines()
            .map(|line| line.split(',').nth(13).unwrap().parse().unwrap())
            .collect();

        let cats: Vec<u32> = self
            .gatya_data
            .as_ref()
            .unwrap()
            .lines()
            .nth(self.id as usize)
            .unwrap()
            .split(',')
            .filter(|s| *s != "-1" && !s.is_empty())
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        for cat in &cats {
            let name = self.get_cat_name(*cat).await?;
            cat_ids[rarities[*cat as usize] as usize].push((*cat as i32, name));
        }

        Ok(cat_ids)
    }
}
