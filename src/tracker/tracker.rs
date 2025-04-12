use super::rand::{get_rarity_id, rand};
use crate::config::structs::EventData;
use crate::functions::logger::logger::{log_gatya, RareLevel};

impl EventData {
    pub async fn print_track(&self, mut seed: u32) -> Result<(), Box<dyn std::error::Error>> {
        let cats = self.get_cat_ids().await?;

        for num in 0..15 {
            let result_first = rand(seed)?;
            let rarity_id = get_rarity_id(result_first.1);

            let result_second = rand(result_first.0)?;

            let rarity_cats = &cats[rarity_id as usize];
            let rarity_len = &rarity_cats.len();
            let modsize = result_second.0 as usize % rarity_len;

            let message = format!(
                "Len: {} Rarity_Seed: {} Rarity: {} Mod: {}",
                rarity_len, result_first.1, rarity_id, modsize
            );

            match rarity_id {
                5 => log_gatya(RareLevel::Legend, message),
                4 => log_gatya(RareLevel::UberRare, message),
                3 => log_gatya(RareLevel::SuperRare, message),
                2 => log_gatya(RareLevel::Rare, message),
                1 => log_gatya(RareLevel::EX, message),
                _ => log_gatya(RareLevel::Noamal, message),
            }

            let id = &rarity_cats[modsize];
            println!("{:?} {:?}", result_first, result_second);
            println!("No.{:<4} ID:{:<6} Name: {}", num + 1, id.0, id.1);
            println!();

            seed = result_second.0;
        }

        Ok(())
    }
}
