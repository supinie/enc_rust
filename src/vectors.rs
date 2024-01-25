use core::num::TryFromIntError;

use crate::{
    matrix::{Mat1024, Mat512, Mat768},
    params::{Eta, GetSecLevel, SecurityLevel, K, POLYBYTES},
    polynomials::Poly,
};
use tinyvec::ArrayVec;

pub type PolyVec512 = ArrayVec<[Poly; 2]>;
pub type PolyVec768 = ArrayVec<[Poly; 3]>;
pub type PolyVec1024 = ArrayVec<[Poly; 4]>;

impl GetSecLevel for PolyVec512 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Two)
    }
}

impl GetSecLevel for PolyVec768 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Three)
    }
}

impl GetSecLevel for PolyVec1024 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Four)
    }
}

trait SameSecLevel {}

pub trait PolyVecOperations {
    fn add(&mut self, addend: Self);
    fn barrett_reduce(&mut self);
    fn normalise(&mut self);
    fn ntt(&mut self);
    fn inv_ntt(&mut self);
    fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta);
    fn pack(&self, buf: &mut [u8]);
    fn unpack(&mut self, buf: &[u8]);
    fn compress(&self, buf: &mut [u8]) -> Result<(), TryFromIntError>;
    fn decompress(&mut self, buf: &[u8]) -> Result<(), TryFromIntError>;
}

macro_rules! impl_polyvec {
    ($variant:ty) => {
        impl PolyVecOperations for $variant {
            fn add(&mut self, addend: Self) {
                for (augend_poly, addend_poly) in self.iter_mut().zip(addend.iter()) {
                    augend_poly.add(&addend_poly);
                }
            }

            fn barrett_reduce(&mut self) {
                for poly in self.iter_mut() {
                    poly.barrett_reduce();
                }
            }

            fn normalise(&mut self) {
                for poly in self.iter_mut() {
                    poly.normalise();
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

            // buf should be of length K * POLYBYTES
            fn pack(&self, buf: &mut [u8]) {
                for (k, poly) in self.iter().enumerate() {
                    poly.pack(&mut buf[k * POLYBYTES..(k + 1) * POLYBYTES]);
                }
            }

            fn unpack(&mut self, buf: &[u8]) {
                for (k, poly) in self.iter_mut().enumerate() {
                    poly.unpack(&buf[k * POLYBYTES..(k + 1) * POLYBYTES]);
                }
            }

            // buf should be of length k * poly_compressed_bytes
            fn compress(&self, buf: &mut [u8]) -> Result<(), TryFromIntError> {
                for (k, poly) in self.iter().enumerate() {
                    poly.compress(
                        &mut buf[k * <$variant as GetSecLevel>::sec_level().poly_compressed_bytes()
                            ..(k + 1)
                                * <$variant as GetSecLevel>::sec_level().poly_compressed_bytes()],
                        &<$variant as GetSecLevel>::sec_level(),
                    )?;
                }
                Ok(())
            }

            fn decompress(&mut self, buf: &[u8]) -> Result<(), TryFromIntError> {
                for (k, poly) in self.iter_mut().enumerate() {
                    poly.decompress(
                        &buf[k * <$variant as GetSecLevel>::sec_level().poly_compressed_bytes()
                            ..(k + 1)
                                * <$variant as GetSecLevel>::sec_level().poly_compressed_bytes()],
                        &<$variant as GetSecLevel>::sec_level(),
                    )?;
                }
                Ok(())
            }
        }
        impl SameSecLevel for $variant {}
    };
}

impl_polyvec!(PolyVec512);
impl_polyvec!(PolyVec768);
impl_polyvec!(PolyVec1024);

pub trait LinkSecLevel<P: PolyVecOperations> {}
impl LinkSecLevel<PolyVec512> for Mat512 {}
impl LinkSecLevel<PolyVec768> for Mat768 {}
impl LinkSecLevel<PolyVec1024> for Mat1024 {}

impl Poly {
    pub fn inner_product_pointwise<T>(&mut self, multiplicand: T, multiplier: T)
    where
        T: PolyVecOperations + IntoIterator<Item = Self>,
    {
        let mut temp = Self::new();
        *self = Self::new(); // Zero output Poly
        for (multiplicand_poly, multiplier_poly) in
            multiplicand.into_iter().zip(multiplier.into_iter())
        {
            temp = multiplicand_poly;
            temp.pointwise_mul(&multiplier_poly);
            self.add(&temp);
        }
    }
}
