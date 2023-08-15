use crate::{
    buffer::Buffer,
    field_ops::*,
    ntt::ZETAS,
    params::{N, Q},
};

#[derive(Copy, Clone)]
pub struct Poly {
    pub coeffs: [i16; N],
}

impl Poly {
    // We can't use default, as that is only supported for arrays of length 32 or less
    pub fn new() -> Self {
        Poly { coeffs: [0; N] }
    }

    // Sets self to self + x
    pub fn add(&mut self, x: &Poly) {
        for i in 0..N {
            self.coeffs[i] += x.coeffs[i];
        }
    }

    // Sets self to self - x
    pub fn sub(&mut self, x: &Poly) {
        for i in 0..N {
            self.coeffs[i] -= x.coeffs[i];
        }
    }

    // Normalise coefficients of given polynomial
    pub fn normalise(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = cond_sub_q(barrett_reduce(*coeff));
        }
    }

    pub fn reduce(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = barrett_reduce(*coeff);
        }
    }

    pub fn to_mont(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = to_mont(*coeff);
        }
    }

    // Pointwise multiplication of two polynomials,
    // assumes inputs are of montgomery form.
    pub fn pointwise_mul(&mut self, x: &Poly) {
        let mut j: usize = 64;

        for i in (0..N).step_by(4) {
            let zeta = ZETAS[j] as i32;
            j += 1;

            let mut p0 = montgomery_reduce((self.coeffs[i + 1] as i32) * (x.coeffs[i + 1] as i32));
            p0 = montgomery_reduce((p0 as i32) * zeta);
            p0 += montgomery_reduce((self.coeffs[i] as i32) * (x.coeffs[i] as i32));

            let mut p1 = montgomery_reduce((self.coeffs[i] as i32) * (x.coeffs[i + 1] as i32));
            p1 += montgomery_reduce((self.coeffs[i + 1] as i32) * (x.coeffs[i] as i32));

            let mut p2 = montgomery_reduce((self.coeffs[i + 3] as i32) * (x.coeffs[i + 3] as i32));
            p2 = -montgomery_reduce((p2 as i32) * zeta);
            p2 += montgomery_reduce((self.coeffs[i + 2] as i32) * (x.coeffs[i + 2] as i32));

            let mut p3 = montgomery_reduce((self.coeffs[i + 2] as i32) * (x.coeffs[i + 3] as i32));
            p3 += montgomery_reduce((self.coeffs[i + 3] as i32) * (x.coeffs[i + 2] as i32));

            self.coeffs[i] = p0;
            self.coeffs[i + 1] = p1;
            self.coeffs[i + 2] = p2;
            self.coeffs[i + 3] = p3;
        }
    }

    // Unpacks a buffer of bytes into a polynomial
    pub fn unpack(&mut self, buf: Buffer) {
        for i in 0..N / 2 {
            self.coeffs[2 * i] =
                (buf.data[3 * i] as i16) | (((buf.data[3 * i + 1] as i16) << 8) & 0xfff);
            self.coeffs[2 * i + 1] =
                ((buf.data[3 * i + 1] >> 4) as i16) | ((buf.data[3 * i + 2] as i16) << 4);
        }
    }

    pub fn from_msg(&mut self, msg: Buffer) {
        for i in 0..N / 8 {
            for j in 0..8 {
                let mask = (((msg.data[i] as i16) >> j) & 1).wrapping_neg();
                self.coeffs[8 * i + j] = mask & ((Q + 1) / 2) as i16;
            }
        }
    }

    pub fn decompress(&mut self, buf: &Buffer, compressed_bytes: usize) {
        let mut k = 0usize;

        match compressed_bytes {
            128 => {
                for i in 0..N / 2 {
                    self.coeffs[2 * i] = ((((buf.data[k] & 15) as usize) * Q + 8) >> 4) as i16;
                    self.coeffs[2 * i + 1] = ((((buf.data[k] >> 4) as usize) * Q + 8) >> 4) as i16;
                    k += 1;
                }
            }
            160 => {
                let mut t = [0u8; 8];
                for i in 0..N / 8 {
                    t[0] = buf.data[k];
                    t[1] = (buf.data[k] >> 5) | (buf.data[k + 1] << 3);
                    t[2] = buf.data[k + 1] >> 2;
                    t[3] = (buf.data[k + 1] >> 7) | (buf.data[k + 2] << 1);
                    t[4] = (buf.data[k + 2] >> 4) | (buf.data[k + 3] << 4);
                    t[5] = buf.data[k + 3] >> 1;
                    t[6] = (buf.data[k + 3] >> 6) | (buf.data[k + 4] << 2);
                    t[7] = buf.data[k + 4] >> 3;
                    k += 5;

                    for j in 0..8 {
                        self.coeffs[8 * i + j] =
                            ((((t[j] as u32) & 31) * (Q as u32) + 16) >> 5) as i16;
                    }
                }
            }
            _ => panic!("Invalid compressed poly bytes size."),
        }
    }
}
