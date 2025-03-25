use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};

use super::write::BCUZIP;

impl BCUZIP {
    pub fn aes_pack(
        &self,
        length: usize,
        data: &[u8],
    ) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            &self.key,
            &self.iv,
            blockmodes::NoPadding,
        );

        let mut final_result = Vec::with_capacity(data.len());
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0u8; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        while let Ok(status) = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            final_result.extend(write_buffer.take_read_buffer().take_remaining());
            if matches!(status, BufferResult::BufferUnderflow) {
                break;
            }
        }

        let result_length = length.min(final_result.len());
        final_result.truncate(result_length);

        Ok(final_result)
    }
}
