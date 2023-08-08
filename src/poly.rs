use crate::{params::N, field_ops::*};

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

    // In place Cooley-Tukey radix-2 Decimation in Time (DIT) NTT algorithm
    pub fn ntt(&mut self) {
        let mut k = 0usize;
        let mut l = N / 2;
        while l > 1 {
            let mut offset = 0;
            while offset < N - l {
                k+=1;
                let zeta = ZETAS[k] as i32;

                let mut j = offset;
                while j < offset + l {
                    let t = montgomery_reduce(zeta * (self.coeffs[j + l] as i32));
                    self.coeffs[j + l] = self.coeffs[j] - t;
                    self.coeffs[j] += t;
                    j += 1;
                }
                offset += 2 * l;
            }
            l >>= 1;
        }
    }

    // In place inverse NTT, with montgomery reduction
    pub fn inv_ntt(&mut self) {
        let mut k: usize = 127;
        let mut r: isize = -1;
        let mut l = 2;

        while l < N {
            for offset in (0..N - 1).step_by(2 * l) {
                let min_zeta = ZETAS[k] as i32;
                k -= 1;

                for j in offset..offset + l {
                    let t = self.coeffs[j + l] - self.coeffs[j];
                    self.coeffs[j] += self.coeffs[j + l];
                    self.coeffs[j + l] = montgomery_reduce(min_zeta * (t as i32));
                }
            }

            loop {
                r += 1;
                let i = INV_NTT_REDUCTIONS[r as usize];
                if i < 0 {
                    break;
                }
                self.coeffs[i as usize] = barrett_reduce(self.coeffs[i as usize]);
            }
            l <<= 1;
        }

        for j in 0..N {
            self.coeffs[j] = montgomery_reduce(1441 * (self.coeffs[j] as i32));
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
}
