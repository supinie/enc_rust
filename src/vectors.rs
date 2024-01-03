use crate::polynomials::Poly;
use crate::params::{K, Eta};
use arrayvec::ArrayVec;

pub(crate) type PolyVec512 = ArrayVec<Poly, 2>;
pub(crate) type PolyVec768 = ArrayVec<Poly, 3>;
pub(crate) type PolyVec1024 = ArrayVec<Poly, 4>;

trait SameSecLevel {}

pub(crate) trait PolyVec{
    fn add(&mut self, addend: Self);
    fn reduce(&mut self);
    fn normalise(&mut self);
    fn ntt(&mut self);
    fn inv_ntt(&mut self);
    fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta);
}


macro_rules! impl_polyvec {
    ($variant:ty) => {
        impl PolyVec for $variant {
            fn add(&mut self, addend: Self) {
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

            fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta) {
                for poly in self.iter_mut() {
                    poly.derive_noise(seed, nonce, eta);
                }
            }
        }
        impl SameSecLevel for $variant {}
    }
}

impl_polyvec!(PolyVec512);
impl_polyvec!(PolyVec768);
impl_polyvec!(PolyVec1024);


impl Poly {
    pub(crate) fn inner_product_pointwise<T>(&mut self, multiplicand: T, multiplier: T)
    where
        T: PolyVec + IntoIterator<Item = Poly>,
    {
        let mut temp = Self::new();
        *self = Self::new();    // Zero output Poly
        for (multiplicand_poly, multiplier_poly) in multiplicand.into_iter().zip(multiplier.into_iter()) {
            temp = multiplicand_poly;
            temp.pointwise_mul(&multiplier_poly);
            self.add(&temp);
        }
    }
}
