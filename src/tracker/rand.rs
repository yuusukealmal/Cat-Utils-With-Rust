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

pub fn get_rarity_id(mut seed: u32) -> u32 {
    const MAX: u32 = 10000;
    const LEGEND_PROB: u32 = 60;
    const UBER_PROB: u32 = 500;
    const SUPER_RARE_PROB: u32 = 2500;

    const LEGEND_CHANCE: u32 = MAX - LEGEND_PROB;
    const UBER_CHANCE: u32 = LEGEND_CHANCE - UBER_PROB;
    const SUPER_RARE_CHANCE: u32 = UBER_CHANCE - SUPER_RARE_PROB;

    if seed >= MAX {
        seed = seed % MAX;
    }

    if seed >= LEGEND_CHANCE {
        return 5;
    } else if seed >= UBER_CHANCE {
        return 4;
    } else if seed >= SUPER_RARE_CHANCE {
        return 3;
    } else {
        return 2;
    }
}
