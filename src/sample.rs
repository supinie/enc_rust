use crate::{params::*, poly::*};

use sha3::{digest::{Update, ExtendableOutput, XofReader}, Shake256};
use byteorder::{ByteOrder, LittleEndian};

impl Poly {
    pub fn derive_noise_2(&mut self, seed: &[u8], nonce: u8) {
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
                let a = (d as i16) & 0x3;
                d >>= 2;
                let b = (d as i16) & 0x3;
                d >>= 2;
                self.coeffs[16 * i + j] = a - b;
            }
        }
    }

    pub fn derive_noise_3(&mut self, seed: &[u8], nonce: u8) {
        let key_suffix: [u8; 1] = [nonce];
        let mut h = Shake256::default();
        h.update(seed);
        h.update(&key_suffix);

        let mut buf = [0u8; 192 + 2];
        h.finalize_xof().read(&mut buf);

        for i in 0..32 {
            let t_bytes = &buf[i * 6..(i + 1) * 6];
            let t = LittleEndian::read_u64(t_bytes);

            let mut d = t & 0x249249249249;
            d += (t >> 1) & 0x249249249249;
            d += (t >> 2) & 0x249249249249;

            for j in 0..8 {
                let a = (d as i16) & 0x7;
                d >>= 3;
                let b = (d as i16) & 0x7;
                d >>= 3;
                self.coeffs[8 * i + j] = a - b;
            }
        }
    }
}
