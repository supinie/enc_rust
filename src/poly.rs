use crate::{params::N, field_ops::*, ntt::ZETAS, buffer::Buffer};

#[derive(Copy, Clone)]
pub struct Poly {
    pub coeffs: [i16; N]
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

    pub fn barrett_reduce(&mut self) {
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

            let mut p0 = montgomery_reduce((self.coeffs[i+1] as i32) * (x.coeffs[i+1] as i32));
            p0 = montgomery_reduce((p0 as i32) * zeta);
            p0 += montgomery_reduce((self.coeffs[i] as i32) * (x.coeffs[i] as i32));

            let mut p1 = montgomery_reduce((self.coeffs[i] as i32) * (x.coeffs[i+1] as i32));
            p1 += montgomery_reduce((self.coeffs[i+1] as i32) * (x.coeffs[i] as i32));

            let mut p2 = montgomery_reduce((self.coeffs[i+3] as i32) * (x.coeffs[i+3] as i32));
            p2 = - montgomery_reduce((p2 as i32) * zeta);
            p2 += montgomery_reduce((self.coeffs[i+2] as i32) * (x.coeffs[i+2] as i32));

            let mut p3 = montgomery_reduce((self.coeffs[i+2] as i32) * (x.coeffs[i+3] as i32));
            p3 += montgomery_reduce((self.coeffs[i+3] as i32) * (x.coeffs[i+2] as i32));

            self.coeffs[i] = p0;
            self.coeffs[i + 1] = p1;
            self.coeffs[i + 2] = p2;
            self.coeffs[i + 3] = p3;
        }
    }

    // Packs given poly into buf
    pub fn unpack(&mut self, buf: &mut Buffer) {
        for i in 0..N/2 {
            let t0 = self.coeffs[2*i];
            let t1 = self.coeffs[2*i + 1];
            
            buf.data[3*i] = t0 as u8;
            buf.data[3*i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            buf.data[3*i + 2] = (t1 >> 4) as u8;
        }
    }
}
