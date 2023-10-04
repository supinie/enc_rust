use core::num::TryFromIntError;
use crate::{params::*, poly::*};
pub trait Buffer {
    fn pack(&mut self, poly: Poly);
    fn msg_from_poly(&mut self, poly: Poly) -> Result<(), TryFromIntError>;
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
}
