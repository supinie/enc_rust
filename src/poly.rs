use crate::{params::N, field_ops::*};

#[derive(Copy, Clone)]
pub struct Poly {
    pub coeffs: [i16; N]
}

impl Poly {
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
        let mut r = 0usize;
        let mut l = 2;

        while l < N {
            let mut offset = 0usize;
            while offset < N - l {
                let min_zeta = ZETAS[k] as i32;
                k-=1;

                let mut j = offset;
                while j < offset + l {
                    let t = self.coeffs[j + l] - self.coeffs[j];
                    self.coeffs[j] += self.coeffs[j + l];
                    self.coeffs[j + l] = montgomery_reduce(min_zeta * (t as i32));

                    j+=1;
                }
                offset += 2 * l;
            }

            let mut i = INV_NTT_REDUCTIONS[r];
            while i >= 0 {
                self.coeffs[i as usize] = barrett_reduce(self.coeffs[i as usize]);
                i = INV_NTT_REDUCTIONS[r];
                r+=1;
            }

            let mut j = 0usize;
            while j < N {
                self.coeffs[j] = montgomery_reduce(1441 * (self.coeffs[j] as i32));
                j+=1;
            }
            l <<= 1;
        }
    }
}
