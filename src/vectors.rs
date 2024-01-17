use core::num::TryFromIntError;

use crate::params::{Eta, SecurityLevel, K, POLYBYTES};
use crate::polynomials::Poly;
pub use arrayvec::ArrayVec as PolyVec;

pub type PolyVec512 = PolyVec<Poly, 2>;
pub type PolyVec768 = PolyVec<Poly, 3>;
pub type PolyVec1024 = PolyVec<Poly, 4>;

trait SameSecLevel {}

pub trait PolyVecOperations {
    fn add(&mut self, addend: Self);
    fn reduce(&mut self);
    fn normalise(&mut self);
    fn ntt(&mut self);
    fn inv_ntt(&mut self);
    fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta);
    fn pack(&self, buf: &mut [u8]);
    fn unpack(&mut self, buf: &[u8]);
    fn compress(&self, buf: &mut [u8], sec_level: &SecurityLevel) -> Result<(), TryFromIntError>;
    fn decompress(&mut self, buf: &[u8], sec_level: &SecurityLevel) -> Result<(), TryFromIntError>;
}

macro_rules! impl_polyvec {
    ($variant:ty) => {
        impl PolyVecOperations for $variant {
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

            fn compress(
                &self,
                buf: &mut [u8],
                sec_level: &SecurityLevel,
            ) -> Result<(), TryFromIntError> {
                for (k, poly) in self.iter().enumerate() {
                    poly.compress(
                        &mut buf[k * sec_level.poly_compressed_bytes()
                            ..(k + 1) * sec_level.poly_compressed_bytes()],
                        sec_level,
                    )?;
                }
                Ok(())
            }

            fn decompress(
                &mut self,
                buf: &[u8],
                sec_level: &SecurityLevel,
            ) -> Result<(), TryFromIntError> {
                for (k, poly) in self.iter_mut().enumerate() {
                    poly.decompress(
                        &buf[k * sec_level.poly_compressed_bytes()
                            ..(k + 1) * sec_level.poly_compressed_bytes()],
                        sec_level,
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

impl Poly {
    pub fn hadamard_flatten<T>(&mut self, multiplicand: T, multiplier: T)
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
