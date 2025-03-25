pub mod aes_decrypt {
    use std::{env, error::Error};

    use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
    use crypto::{aes, blockmodes, buffer};

    pub fn decrypt_ecb(is_pack: bool, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let key = if is_pack {
            env::var("PACK").map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Key for PACK not found"),
                )
            })?
        } else {
            env::var("LIST").map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Key for LIST not found"),
                )
            })?
        };

        let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key.as_bytes(),
            blockmodes::PkcsPadding,
        );

        let mut result = Vec::with_capacity(data.len());
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0u8; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        while let Ok(status) = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            result.extend(write_buffer.take_read_buffer().take_remaining());
            if matches!(status, BufferResult::BufferUnderflow) {
                break;
            }
        }

        Ok(result)
    }

    pub fn decrypt_cbc(cc: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let key = env::var(format!("{}_KEY", cc)).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Key for {} not found", cc),
            )
        })?;
        let iv = env::var(format!("{}_IV", cc)).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("IV for {} not found", cc),
            )
        })?;

        let key_bytes = hex::decode(&key).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key format")
        })?;
        let iv_bytes = hex::decode(&iv).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid IV format")
        })?;

        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            &key_bytes,
            &iv_bytes,
            blockmodes::NoPadding,
        );

        let mut result = Vec::with_capacity(data.len());
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0u8; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        while let Ok(status) = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            result.extend(write_buffer.take_read_buffer().take_remaining());
            if matches!(status, BufferResult::BufferUnderflow) {
                break;
            }
        }

        Ok(delete_padding(&result).to_vec())
    }

    fn delete_padding(res: &[u8]) -> &[u8] {
        if let Some(&last_byte) = res.last() {
            let padding_count = if last_byte > 0 && last_byte <= 16 {
                last_byte as usize
            } else {
                0
            };

            if padding_count > 0 && padding_count <= res.len() {
                return &res[..res.len() - padding_count];
            }
        }

        res
    }
}
