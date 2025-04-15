use crate::config::structs::EventData;
use crate::functions::logger::logger::RareLevel;

pub fn rand(mut seed: u32) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    fn xor_shift(mut seed: u32) -> u32 {
        seed ^= seed.wrapping_shl(13);
        seed ^= seed.wrapping_shr(17);
        seed ^= seed.wrapping_shl(15);
        seed
    }

    seed = xor_shift(seed);

    Ok((seed, seed % 10000))
}

impl EventData {
    pub fn get_rarity_id(&self, mut seed: u32) -> RareLevel {
        const MAX: u32 = 10000;
        let legend_chance: u32 = MAX - self.legend;
        let uber_chance: u32 = legend_chance - self.uber_rare;
        let super_rare_chance: u32 = uber_chance - self.super_rare;

        if seed >= MAX {
            seed = seed % MAX;
        }

        if seed >= legend_chance {
            return RareLevel::Legend;
        } else if seed >= uber_chance {
            return RareLevel::UberRare;
        } else if seed >= super_rare_chance {
            return RareLevel::SuperRare;
        } else {
            return RareLevel::Rare;
        }
    }
}
