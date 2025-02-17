use crate::{
    errors::CrystalsError,
    params::{Eta, N, Q, SYMBYTES},
    polynomials::{Montgomery, Poly},
};
use byteorder::{ByteOrder, LittleEndian};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128, Shake256,
};

impl Poly<Montgomery> {
    // Sample our polynomial from a centered binomial distribution
    // given a uniformly distributed array of bytes
    // n = 4, p = 1/2
    // ie. coefficients are in {-2, -1, 0, 1, 2}
    // with probabilities {1/16, 1/4, 3/8, 1/4, 1/16}
    fn derive_noise_2(seed: &[u8], nonce: u8) -> Self {
        let key_suffix: [u8; 1] = [nonce];
        let mut hash = Shake256::default();
        hash.update(seed);
        hash.update(&key_suffix);

        let mut entropy_buf = [0u8; SYMBYTES * 4];
        hash.finalize_xof().read(&mut entropy_buf);

        let mut coeffs = [0i16; N];

        for (i, coeff_bytes) in entropy_buf.chunks_exact(8).enumerate() {
            let coeff_sum = LittleEndian::read_u64(coeff_bytes);

            let mut accumulated_sum = coeff_sum & 0x5555_5555_5555_5555;
            accumulated_sum += (coeff_sum >> 1) & 0x5555_5555_5555_5555;

            #[allow(clippy::cast_possible_truncation)]
            for coeff in coeffs.iter_mut().skip(16 * i).take(16) {
                let coeff_a = (accumulated_sum as i16) & 0x3;
                accumulated_sum >>= 2;
                let coeff_b = (accumulated_sum as i16) & 0x3;
                accumulated_sum >>= 2;
                *coeff = coeff_a - coeff_b;
            }
        }

        Self {
            coeffs,
            state: Montgomery,
        }
    }

    // Sample our polynomial from a centered binomial distribution
    // n = 6, p = 1/2
    // ie. coefficients are in {-3, -2, -1, 0, 1, 2, 3}
    // with probabilities {1/64, 3/32, 15/64, 5/16, 15/64, 3/32, 1/64}
    fn derive_noise_3(seed: &[u8], nonce: u8) -> Self {
        let key_suffix: [u8; 1] = [nonce];
        let mut hash = Shake256::default();
        hash.update(seed);
        hash.update(&key_suffix);

        let mut entropy_buf = [0u8; SYMBYTES * 6];
        hash.finalize_xof().read(&mut entropy_buf);

        let mut coeffs = [0i16; N];

        for (i, coeff_bytes) in entropy_buf.chunks_exact(6).enumerate() {
            //must be able to read 8 bytes even though we only use 6.
            let coeff_sum = LittleEndian::read_u64(&[coeff_bytes, &[0u8; 2]].concat());

            let mut accumulated_sum = coeff_sum & 0x2492_4924_9249;
            accumulated_sum += (coeff_sum >> 1) & 0x2492_4924_9249;
            accumulated_sum += (coeff_sum >> 2) & 0x2492_4924_9249;

            #[allow(clippy::cast_possible_truncation)]
            for coeff in coeffs.iter_mut().skip(8 * i).take(8) {
                let coeff_a = (accumulated_sum as i16) & 0x7;
                accumulated_sum >>= 3;
                let coeff_b = (accumulated_sum as i16) & 0x7;
                accumulated_sum >>= 3;

                *coeff = coeff_a - coeff_b;
            }
        }

        Self {
            coeffs,
            state: Montgomery,
        }
    }

    pub(crate) fn derive_noise(seed: &[u8], nonce: u8, eta: Eta) -> Self {
        match eta {
            Eta::Two => Self::derive_noise_2(seed, nonce),
            Eta::Three => Self::derive_noise_3(seed, nonce),
        }
    }

    // seed should be of length 32
    // coefficients are reduced, but not normalised (close to normal) {0..q}
    pub(crate) fn derive_uniform(seed: &[u8], x: u8, y: u8) -> Result<Self, CrystalsError> {
        if seed.len() != 32 {
            return Err(CrystalsError::InvalidSeedLength(seed.len(), 32));
        }
        let seed_suffix = [x, y];
        let mut buf = [0u8; 168];

        let mut coeffs = [0i16; N];

        let mut i = 0;
        let mut hash = Shake128::default();
        hash.update(seed);
        hash.update(&seed_suffix);
        let mut reader = hash.finalize_xof();

        'outer: loop {
            reader.read(&mut buf);

            let chunk_iter = buf.chunks_exact_mut(3);
            for chunk in chunk_iter {
                let t1 = (u16::from(chunk[0]) | (u16::from(chunk[1]) << 8)) & 0xfff;
                let t2 = ((u16::from(chunk[1]) >> 4) | (u16::from(chunk[2]) << 4)) & 0xfff;

                #[allow(clippy::cast_possible_wrap)]
                if usize::from(t1) < Q {
                    coeffs[i] = t1 as i16;
                    i += 1;
                    if i == N {
                        break 'outer;
                    }
                }

                #[allow(clippy::cast_possible_wrap)]
                if usize::from(t2) < Q {
                    coeffs[i] = t2 as i16;
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

        Ok(Self {
            coeffs,
            state: Montgomery,
        })
    }
}
