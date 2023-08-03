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

            while true {
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
        for coefficient in self.coeffs.iter_mut() {
            *coefficient = cond_sub_q(barrett_reduce(*coefficient));
        }
    }

}
