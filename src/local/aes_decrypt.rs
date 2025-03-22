pub mod aes_decrypt {
    use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
    use crypto::{aes, blockmodes, buffer, symmetriccipher};

    pub fn decrypt_list(
        key: &[u8],
        data: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor =
            aes::ecb_decryptor(aes::KeySize::KeySize128, key, blockmodes::PkcsPadding);

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

    pub fn decrypt_pack(
        key: &str,
        iv: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            hex::decode(key).unwrap().as_slice(),
            hex::decode(iv).unwrap().as_slice(),
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
