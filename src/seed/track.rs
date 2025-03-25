use super::handle::SaveParser;
use crate::functions::logger::logger::{log, LogLevel};

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
            let shift = i << 7;
            let val = self.seek(1);
            i = shift | (val & 127);
            if (val & 128) == 0 {
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

        let oldadd = self.address;
        let newadd = self.find_date();
        self.set_address(oldadd);
        self.seek(newadd - oldadd);

        if dst == None {
            dst = Some(self.get_dst(self.address + 118));
        }

        self.seek(44);
        if dst.unwrap() == true {
            self.seek(1);
        }

        self.seek(73);

        let val = self.seek(1) as usize;
        self.seek(val * 40 + 4212);

        let data = self.get_data(4, 4, None);
        if data.is_empty() {
            return self.parse_save(Some(!dst.unwrap_or(false)));
        }

        self.get_data(4, 4, None);

        let val = self.seek(4) as usize;
        self.get_data(4, 2, Some(val * 2));
        self.get_data(4, 4, None);
        self.seek(44);
        self.get_data(4, 4, None);
        self.get_data(4, 4, None);
        self.seek(24);
        self.get_data(4, 4, None);

        self.seek(95);

        self.get_time_stamp(dst);
        self.seek(200);
        self.get_time_stamp(dst);
        self.seek(24);

        self.get_data(4, 1, None);

        let val = self.seek_var();
        for _ in 0..val {
            self.seek_var();
            self.seek_var();
        }
        let val = self.seek_var();
        for _ in 0..val {
            self.seek_var();
            self.seek(1);
        }

        self.seek(24);
        self.get_data(4, 4, None);
        self.seek(40);

        let mut val = self.seek(2);
        if val != 128 {
            self.set_address(self.address - 2);
            val = 100;
        }

        self.seek(val as usize * 8);

        let val = self.seek(1);
        let a = self.seek(2) * val;
        let b = self.seek(1);
        let c = self.seek(1);
        self.seek((a * b) as usize * 2);
        self.seek((a * c * b) as usize * 2);
        self.seek((a * b) as usize);
        self.seek(152);
        self.get_data(4, 4, None);

        let seed = self.seek(4);

        log(LogLevel::Info, format!("Get Seed: {}", seed));
        seed
    }
}
