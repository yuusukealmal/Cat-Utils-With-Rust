use super::rand::rand;
use crate::config::structs::{Cat, EventData};
use crate::functions::logger::logger::{log_gatya, RareLevel};

impl Cat {
    fn unknown() -> Self {
        Self {
            id: 0,
            name: "Unknown".to_string(),
            rarity: RareLevel::Normal,
            seed: (0, 0),
        }
    }
}

impl EventData {
    fn fetch(&self, seed: u32) -> Result<Cat, Box<dyn std::error::Error>> {
        let result_first = rand(seed)?;
        let rarity = self.get_rarity_id(result_first.1);

        let result_second = rand(result_first.0)?;
        let cats = self
            .cat_ids
            .as_ref()
            .unwrap()
            .get(rarity.clone() as usize)
            .unwrap();
        let id = cats[result_second.0 as usize % cats.len()].clone();

        Ok(Cat {
            id: id.0,
            name: id.1,
            rarity,
            seed: (result_first.0, result_second.0),
        })
    }

    pub async fn print_track(&self, seed: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut seed1 = seed;
        let mut seed2 = rand(seed)?.0;
        let mut track1 = vec![];
        let mut track2 = vec![];

        for _ in 0..30 {
            track1.push(self.fetch(seed1)?);
            seed1 = track1.last().unwrap().seed.1;

            track2.push(self.fetch(seed2)?);
            seed2 = track2.last().unwrap().seed.1;
        }

        let mut result = vec![track1[0].clone()];
        let mut index = 1;
        let mut is_track1 = true;

        while result.len() < 10 {
            let (current, previous, alt_track) = if is_track1 {
                (&track1[index], &track1[index - 1], &track2)
            } else {
                (&track2[index], &track2[index - 1], &track1)
            };

            if current.id == previous.id && current.rarity == RareLevel::Rare {
                index += if is_track1 { 1 } else { 2 };
                if index < alt_track.len() {
                    result.push(Cat::unknown()); //alt_track[index].clone()
                    is_track1 = !is_track1;
                }
            } else {
                result.push(current.clone());
                index += 1;
            }
        }

        for (i, cat) in result.iter().enumerate() {
            let msg = format!("Seed: {:?} Rarity: {:?}", cat.seed, cat.rarity);

            log_gatya(&cat.rarity, msg);
            println!("No.{:<4} ID:{:<6} Name: {}\n", i + 1, cat.id, cat.name);
        }

        Ok(())
    }
}
