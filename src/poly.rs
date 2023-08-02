use crate::{params::N, field_ops::{ZETAS, montgomery_reduce}};

#[derive(Copy, Clone, Default)]
pub struct Poly {
    pub coeffs: [i16; N]
}

impl Poly {
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
                let zeta = (ZETAS[k] as i32);

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
}
