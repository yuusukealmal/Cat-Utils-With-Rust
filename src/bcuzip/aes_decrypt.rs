pub mod aes {
    use std::result::Result;

    use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
    use crypto::{aes, blockmodes, buffer, symmetriccipher};

    use crate::bcuzip::file_parser::BCUZIP;

    pub fn aes_pack(
        zip: &BCUZIP,
        length: u32,
        data: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            &zip.key,
            &zip.iv,
            blockmodes::NoPadding,
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);

            final_result.extend(write_buffer.take_read_buffer().take_remaining());
            match result {
                Ok(BufferResult::BufferUnderflow) => break,
                Ok(BufferResult::BufferOverflow) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let r = &final_result[0..length as usize];
        Ok(r.to_vec())
    }
}
