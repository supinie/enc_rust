use crate::params::N;

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
}
