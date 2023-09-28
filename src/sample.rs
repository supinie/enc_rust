use crate::{params::*, poly::*};
use core::num::TryFromIntError;
use sha3::{digest::{Update, ExtendableOutput, XofReader}, Shake256};
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub enum DeriveNoiseError {
    TryFromIntError,
    InvalidETAError,
}

impl From<TryFromIntError> for DeriveNoiseError {
    fn from(_err: TryFromIntError) -> Self {
        DeriveNoiseError::TryFromIntError
    }
}

impl Poly {
    // Sample our polynomial from a centered binomial distribution
    // n = 4, p = 1/2
    // ie. coefficients are in {-2, -1, 0, 1, 2}
    // with probabilities {1/16, 1/4, 3/8, 1/4, 1/16}
    pub fn derive_noise_2(&mut self, seed: &[u8], nonce: u8) -> Result<(), TryFromIntError> {
        let key_suffix: [u8; 1] = [nonce];
        let mut h = Shake256::default();
        h.update(seed);
        h.update(&key_suffix);

        let mut buf = [0u8; 128];
        h.finalize_xof().read(&mut buf);

        for i in 0..16 {
            let t_bytes = &buf[i * 8..(i + 1) * 8];
            let t = LittleEndian::read_u64(t_bytes);
            
            let mut d = t & 0x5555555555555555;
            d += (t >> 1) & 0x5555555555555555;

            for j in 0..16 {
                let a = i16::try_from(d)? & 0x3;
                d >>= 2;
                let b = i16::try_from(d)? & 0x3;
                d >>= 2;
                self.coeffs[16 * i + j] = a - b;
            }
        }
        Ok(())
    }
    
    // Sample our polynomial from a centered binomial distribution
    // n = 6, p = 1/2
    // ie. coefficients are in {-3, -2, -1, 0, 1, 2, 3}
    // with probabilities {1/64, 3/32, 15/64, 5/16, 15/64, 3/32, 1/64}
    pub fn derive_noise_3(&mut self, seed: &[u8], nonce: u8) -> Result<(), TryFromIntError> {
        let key_suffix: [u8; 1] = [nonce];
        let mut h = Shake256::default();
        h.update(seed);
        h.update(&key_suffix);

        let mut buf = [0u8; 192 + 2];
        h.finalize_xof().read(&mut buf);

        for i in 0..32 {
            let t_bytes = &buf[i * 6..i * 6 + 8];
            let t = LittleEndian::read_u64(t_bytes);

            let mut d = t & 0x249249249249;
            d += (t >> 1) & 0x249249249249;
            d += (t >> 2) & 0x249249249249;

            for j in 0..8 {
                let a = i16::try_from(d)? & 0x7;
                d >>= 3;
                let b = i16::try_from(d)? & 0x7;
                d >>= 3;
                self.coeffs[8 * i + j] = a - b;
            }
        }
        Ok(())
    }

    pub fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: usize) -> Result<(), DeriveNoiseError> {
        match eta {
            2 => self.derive_noise_2(seed, nonce).map_err(|err| err.into()),
            3 => self.derive_noise_3(seed, nonce).map_err(|err| err.into()),
            _ => Err(DeriveNoiseError::InvalidETAError),
        }
    }
}
