pub mod file_getter {
    use std::{fs, path::PathBuf};

    use crate::config::routes::{TRACK_EVENT_DATA, TRACK_GATYA_SET, TRACK_UNITBUY};
    use crate::config::structs::EventData;
    use crate::functions::utils::get_folder_name;
    use crate::tracker::event_reader::get_event_lists;

    pub async fn gatya_info(cc: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        let res_path = PathBuf::from(std::env::current_dir()?)
            .join("Data")
            .join(get_folder_name(cc))
            .join("local")
            .join("DataLocal");

        let unitbuy = res_path.join("unitbuy.csv");
        let gatya_data_set = res_path.join("GatyaDataSetR1.csv");

        let (unitbuy_daya, gatya_data) = match unitbuy.exists()
            && unitbuy.is_file()
            && gatya_data_set.exists()
            && gatya_data_set.is_file()
        {
            true => {
                let unitbuy_file = fs::read_to_string(unitbuy)?;
                let gatya_set_file = fs::read_to_string(gatya_data_set)?;

                (unitbuy_file, gatya_set_file)
            }
            false => {
                println!("File Not Found At Cwd, Get From Github.");
                let unitbuy_data = reqwest::get(TRACK_UNITBUY(cc)).await?.text().await?;
                let gatya_set_data = reqwest::get(TRACK_GATYA_SET(cc)).await?.text().await?;

                (unitbuy_data, gatya_set_data)
            }
        };

        Ok((unitbuy_daya, gatya_data))
    }

    pub async fn event_info(cc: &str) -> Result<Vec<EventData>, Box<dyn std::error::Error>> {
        let event_path = PathBuf::from(std::env::current_dir()?)
            .join("Data")
            .join(get_folder_name(cc))
            .join("event")
            .join("gatya.tsv");

        let events = match event_path.exists() && event_path.is_file() {
            true => get_event_lists(fs::read_to_string(event_path)?)?,
            false => {
                let event_data = reqwest::get(TRACK_EVENT_DATA(cc)).await?.text().await?;
                get_event_lists(event_data)?
            }
        };

        Ok(events)
    }
}
