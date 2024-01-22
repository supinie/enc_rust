use crate::{
    params::{Eta, N, Q},
    polynomials::Poly,
};
use byteorder::{ByteOrder, LittleEndian};
use core::num::TryFromIntError;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128, Shake256,
};

impl Poly {
    // Sample our polynomial from a centered binomial distribution
    // n = 4, p = 1/2
    // ie. coefficients are in {-2, -1, 0, 1, 2}
    // with probabilities {1/16, 1/4, 3/8, 1/4, 1/16}
    pub(crate) fn derive_noise_2(&mut self, seed: &[u8], nonce: u8) {
        let key_suffix: [u8; 1] = [nonce];
        let mut hash = Shake256::default();
        hash.update(seed);
        hash.update(&key_suffix);

        let mut entropy_buf = [0u8; 128];
        hash.finalize_xof().read(&mut entropy_buf);

        for i in 0..16 {
            let coeff_bytes = &entropy_buf[i * 8..(i + 1) * 8];
            let coeff_sum = LittleEndian::read_u64(coeff_bytes);

            let mut accumulated_sum = coeff_sum & 0x5555_5555_5555_5555;
            accumulated_sum += (coeff_sum >> 1) & 0x5555_5555_5555_5555;

            #[allow(clippy::cast_possible_truncation)]
            for j in 0..16 {
                let coeff_a = (accumulated_sum as i16) & 0x3;
                accumulated_sum >>= 2;
                let coeff_b = (accumulated_sum as i16) & 0x3;
                accumulated_sum >>= 2;
                self.coeffs[16 * i + j] = coeff_a - coeff_b;
            }
        }
    }

    // Sample our polynomial from a centered binomial distribution
    // n = 6, p = 1/2
    // ie. coefficients are in {-3, -2, -1, 0, 1, 2, 3}
    // with probabilities {1/64, 3/32, 15/64, 5/16, 15/64, 3/32, 1/64}
    pub(crate) fn derive_noise_3(&mut self, seed: &[u8], nonce: u8) {
        let key_suffix: [u8; 1] = [nonce];
        let mut hash = Shake256::default();
        hash.update(seed);
        hash.update(&key_suffix);

        let mut entropy_buf = [0u8; 192 + 2];
        hash.finalize_xof().read(&mut entropy_buf);

        for i in 0..32 {
            let coeff_bytes = &entropy_buf[i * 6..i * 6 + 8];
            let coeff_sum = LittleEndian::read_u64(coeff_bytes);

            let mut accumulated_sum = coeff_sum & 0x2492_4924_9249;
            accumulated_sum += (coeff_sum >> 1) & 0x2492_4924_9249;
            accumulated_sum += (coeff_sum >> 2) & 0x2492_4924_9249;

            #[allow(clippy::cast_possible_truncation)]
            for j in 0..8 {
                let coeff_a = (accumulated_sum as i16) & 0x7;
                accumulated_sum >>= 3;
                let coeff_b = (accumulated_sum as i16) & 0x7;
                accumulated_sum >>= 3;
                self.coeffs[8 * i + j] = coeff_a - coeff_b;
            }
        }
    }

    pub(crate) fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta) {
        match eta {
            Eta::Two => {
                self.derive_noise_2(seed, nonce);
            }
            Eta::Three => {
                self.derive_noise_3(seed, nonce);
            }
        }
    }

    //seed should be of length 32
    pub(crate) fn derive_uniform(&mut self, seed: &[u8], x: u8, y: u8) {
        let mut seed_suffix = [x, y];
        let mut buf = [0u8; 168];

        let mut i = 0;
        'outer: loop {
            let mut hash = Shake128::default();
            hash.update(seed);
            hash.update(&seed_suffix);
            let mut reader = hash.finalize_xof();
            reader.read(&mut buf);

            let mut chunk_iter = buf.chunks_exact_mut(3);
            for chunk in chunk_iter {
                let t1 = (u16::from(chunk[0]) | (u16::from(chunk[1]) << 8)) & 0xfff;
                let t2 = ((u16::from(chunk[1]) >> 4) | (u16::from(chunk[2]) << 4)) & 0xfff;

                #[allow(clippy::cast_possible_wrap)]
                if usize::from(t1) < Q {
                    self.coeffs[i] = t1 as i16;
                    i += 1;
                    if i == N {
                        break 'outer;
                    }
                }

                #[allow(clippy::cast_possible_wrap)]
                if usize::from(t2) < Q {
                    self.coeffs[i] = t2 as i16;
                    i += 1;
                    if i == N {
                        break 'outer;
                    }
                }
                if i == N {
                    break 'outer;
                }
            }
        }
    }
}
