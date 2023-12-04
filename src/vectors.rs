use crate::polynomials::Poly;
use arrayvec::ArrayVec;

pub(crate) type PolyVec = ArrayVec<Poly, 4>;

trait PolyVecMethods{
    fn add(&mut self, addend: &Self);

    fn reduce(&mut self);

    fn normalise(&mut self);

    fn ntt(&mut self);

    fn inv_ntt(&mut self);

    fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: usize) -> Result<(), &str>;
}

impl PolyVecMethods for PolyVec {
    fn add(&mut self, addend: &Self) {
        assert_eq!(self.len(), addend.len());
        for (augend_poly, addend_poly) in self.iter_mut().zip(addend.iter()) {
            augend_poly.add(&addend_poly);
        }
    }

    fn reduce(&mut self) {
        for poly in self.iter_mut() {
            poly.reduce();
        }
    }

    fn normalise(&mut self) {
        for poly in self.iter_mut() {
            poly.reduce();
        }
    }

    fn ntt(&mut self) {
        for poly in self.iter_mut() {
            poly.ntt();
        }
    }

    fn inv_ntt(&mut self) {
        for poly in self.iter_mut() {
            poly.inv_ntt();
        }
    }

    fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: usize) -> Result<(), &str> {
        for poly in self.iter_mut() {
            poly.derive_noise(seed, nonce, eta);
        }
        Ok(())
    }
}

impl Poly {
    pub(crate) fn inner_product_pointwise(&mut self, multiplicand: &PolyVec, multiplier: &PolyVec) {
        assert_eq!(multiplicand.len(), multiplier.len());
        let mut temp = Self::new();
        *self = Self::new();    // Zero output Poly
        for (multiplicand_poly, multiplier_poly) in multiplicand.iter().zip(multiplier.iter()) {
            temp = *multiplicand_poly;
            temp.pointwise_mul(&multiplier_poly);
            self.add(&temp);
        }
    }
}
