use crate::seed::handle::SaveParser;

impl SaveParser {
    pub fn new(save_data: Vec<u8>) -> Self {
        SaveParser {
            address: 0,
            save_data,
        }
    }

    fn set_address(&mut self, offset: usize) {
        self.address = offset;
    }

    fn seek(&mut self, offset: usize) -> u32 {
        let val = self.convert_little(&self.save_data[self.address..self.address + offset]);
        self.address += offset;

        val
    }

    fn convert_little(&self, bytes: &[u8]) -> u32 {
        let mut value = 0u32;
        for (i, &byte) in bytes.iter().take(4).enumerate() {
            value |= (byte as u32) << (8 * i);
        }

        value
    }

    fn get_data(&mut self, offset: usize, number: usize, length: Option<usize>) -> Vec<u32> {
        let len = length.unwrap_or_else(|| self.seek(offset) as usize);
        (0..len).map(|_| self.seek(number)).collect()
    }

    fn find_date(&mut self) -> usize {
        for _ in 0..100 {
            let val = self.seek(4);
            if (2000..=3000).contains(&val) {
                return self.address - 4;
            }
        }
        panic!("Could not find date");
    }

    fn get_dst(&mut self, offset: usize) -> bool {
        let mut dst = false;
        if self.save_data[offset] >= 15 && self.save_data[offset] <= 20 {
            dst = true;
        } else if self.save_data[offset - 1] >= 15 && self.save_data[offset - 1] <= 20 {
            dst = false;
        }

        dst
    }

    fn seek_var(&mut self) -> u32 {
        let mut i = 0;
        for _ in 0..4 {
            let i3 = i << 7;
            let read = self.seek(1);
            i = i3 | (read & 127);
            if (read & 128) == 0 {
                return i;
            }
        }

        i
    }

    fn get_time_stamp(&mut self, dst: Option<bool>) {
        if dst != None {
            self.seek(1);
        }
        self.seek(24);
    }

    pub fn parse_save(&mut self, dst: Option<bool>) -> u32 {
        let mut dst = dst;

        self.seek(15);

        let old_address = self.address;
        let new_address = self.find_date();
        self.set_address(old_address);
        self.seek(new_address - old_address);

        if dst == None {
            dst = Some(self.get_dst(self.address + 118));
        }

        self.seek(44);
        if dst.unwrap() == true {
            self.seek(1);
        }

        self.get_data(4, 4, Some(3));
        self.seek(12);

        self.get_data(4, 4, Some(12));
        self.seek(1);

        let slot_multiplier = self.seek(1) as usize;
        self.get_data(1, 4, Some(slot_multiplier * 10));
        self.seek(4);
        self.get_data(4, 4, Some(30));

        self.seek(8);
        self.get_data(4, 4, Some(10));

        for _ in 0..10 {
            self.seek(4);
        }

        for _ in 0..10 {
            for _ in 0..51 {
                self.seek(4);
            }
        }

        for _ in 0..10 {
            for _ in 0..49 {
                self.seek(4);
            }
        }

        let data = self.get_data(4, 4, None);
        if data.is_empty() {
            return self.parse_save(Some(!dst.unwrap()));
        }

        self.get_data(4, 4, None);

        let cat_upgrades_multiplier = self.seek(4) as usize;
        self.get_data(4, 2, Some(cat_upgrades_multiplier * 2));
        self.get_data(4, 4, None);
        self.get_data(4, 2, Some(11 * 2));
        self.get_data(4, 4, None);
        self.get_data(4, 4, None);
        self.get_data(4, 4, Some(6));
        self.get_data(4, 4, None);

        self.seek(4);
        self.get_data(4, 4, Some(21));
        self.seek(1);
        self.get_data(1, 1, Some(6));

        self.get_time_stamp(dst);
        self.get_data(4, 4, Some(50));
        self.get_time_stamp(dst);
        self.seek(6 * 4);

        self.get_data(4, 1, None);

        let length1 = self.seek_var();
        for _ in 0..length1 {
            self.seek_var();
            self.seek_var();
        }
        let length2 = self.seek_var();
        for _ in 0..length2 {
            self.seek_var();
            self.seek(1);
        }
        self.get_data(4, 4, Some(4));
        self.seek(8);
        self.get_data(4, 4, None);
        self.get_data(4, 4, Some(10));

        let mut len = self.seek(2);
        if len != 128 {
            self.set_address(self.address - 2);
            len = 100;
        }

        self.get_data(2, 4, Some(len as usize));
        self.get_data(2, 4, Some(len as usize));

        let unknown_val = self.seek(1);
        let total_count = self.seek(2) * unknown_val;
        let star_count = self.seek(1);
        let stage_count = self.seek(1);
        self.get_data(1, 1, Some((total_count * star_count) as usize));
        self.get_data(1, 1, Some((total_count * star_count) as usize));
        self.get_data(
            1,
            2,
            Some((total_count * stage_count * star_count) as usize),
        );
        self.get_data(1, 1, Some((total_count * star_count) as usize));
        self.get_data(4, 4, Some(38));
        self.get_data(4, 4, None);

        let seed = self.seek(4);
        seed
    }
}
