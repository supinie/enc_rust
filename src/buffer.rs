use core::num::TryFromIntError;
use crate::{params::*, poly::*};


pub enum CompressError {
    TryFromIntError,
    InvalidCompressedBytes,
}


impl From<TryFromIntError> for CompressError {
    fn from(_err: TryFromIntError) -> Self {
        CompressError::TryFromIntError
    }
}


pub trait Buffer {
    fn pack(&mut self, poly: Poly);
    fn msg_from_poly(&mut self, poly: Poly) -> Result<(), TryFromIntError>;
    fn compress(&mut self, poly: Poly, compressed_bytes: usize) -> Result<(), DecompressError>;
}


impl Buffer for [u8] {
    // Packs given poly into a 384-byte buffer
    // Example:
    // buf.pack(poly);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn pack(&mut self, poly: Poly) {
        for i in 0..N / 2 {
            let t0 = poly.coeffs[2 * i];
            let t1 = poly.coeffs[2 * i + 1];
            
            self[3 * i] = t0 as u8;
            self[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            self[3 * i + 2] = (t1 >> 4) as u8;
        }
    }

    // Convert a given polynomial into a 32-byte message
    // Example:
    // msg.msg_from_poly(poly);
    fn msg_from_poly(&mut self, poly: Poly) -> Result<(), TryFromIntError> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let q_16 = i16::try_from(Q)?;
        for i in 0..N / 8 {
            self[i] = 0;
            for j in 0..8 {
                let mut x = poly.coeffs[8 * i + j];
                x += (x >> 15) & q_16;
                x = (((x << 1) + q_16 / 2) / q_16) & 1;
                self[i] |= u8::try_from(x << j)?;
            }
        }
        Ok(())
    }

    // Compress polynomial to a buffer
    // Example:
    // buf.compress(poly);
    fn compress(&mut self, poly: Poly, compressed_bytes: usize) -> Result<(), DecompressError>{
        let mut k = 0usize;
        let mut t = [0u8; 8];

        match compressed_bytes {
            128 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = poly.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(((((u16::try_from(u)?) << 4) + u16::try_from(Q)? / 2) / u16::try_from(Q)?) & 15)?;
                    }
                    self[k] = t[0] | (t[1] << 4);
                    self[k + 1] = t[2] | (t[3] << 4);
                    self[k + 2] = t[4] | (t[5] << 4);
                    self[k + 3] = t[6] | (t[7] << 4);
                    k += 4;
                }
                Ok(())
            }
            160 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = poly.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(((((u32::try_from(u)?) << 5) + u32::try_from(Q)? / 2) / u32::try_from(Q)?) & 31)?;
                    }
                    self[k] = t[0] | (t[1] << 5);
                    self[k + 1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
                    self[k + 2] = (t[3] >> 1) | (t[4] << 4);
                    self[k + 3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
                    self[k + 4] = (t[6] >> 2) | (t[7] << 3);
                    k += 5;
                }
                Ok(())
            }
            _ => Err(DecompressError::InvalidCompressedBytes)
        }
    }
}
