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
}
