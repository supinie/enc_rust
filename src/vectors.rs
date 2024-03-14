// use core::num::TryFromIntError;
use crate::{
    errors::CrystalsError, 
    params::{SecurityLevel, K}, 
    polynomials::{Normalised, Poly, State, Unnormalised}
};
use tinyvec::array_vec;
use tinyvec::ArrayVec;

#[derive(Default)]
pub struct PolyVec<S: State> {
    polynomials: ArrayVec<[Poly<S>; 4]>,
    sec_level: K,
}

impl<S: State> PolyVec<S> {
    const fn sec_level(&self) -> SecurityLevel {
        SecurityLevel::new(self.sec_level)
    }

    fn polynomials(&self) -> &[Poly<S>] {
        &self.polynomials.as_slice()[..self.sec_level.into()]
    }

    fn add<T: State>(&self, addend: &PolyVec<T>) -> Result<PolyVec<Unnormalised>, CrystalsError> {
        if self.sec_level == addend.sec_level {
            let mut polynomials = ArrayVec::<[Poly<Unnormalised>; 4]>::new();
            for (augend_poly, addend_poly) in self.polynomials.iter().zip(addend.polynomials.iter()) {
                polynomials.push(augend_poly.add(addend_poly));
            }

            Ok(
                PolyVec {
                    polynomials,
                    sec_level: self.sec_level,
                }
            )
        } else {
            Err(CrystalsError::MismatchedSecurityLevels(self.sec_level, addend.sec_level))
        }
    }

    fn barrett_reduce(&self) -> PolyVec<Unnormalised> {
        let mut polynomials = ArrayVec::<[Poly<Unnormalised>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.barrett_reduce());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl PolyVec<Unnormalised> {
    fn normalise(&self) -> PolyVec<Normalised> {
        let mut polynomials = ArrayVec::<[Poly<Normalised>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.normalise());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl PolyVec<Normalised> {
    fn new(k: K) -> Self {
        let polynomials = match k {
            K::Two => array_vec!([Poly<Normalised>; 4] => Poly::new(), Poly::new()),
            K::Three => array_vec!([Poly<Normalised>; 4] => Poly::new(), Poly::new(), Poly::new()),
            K::Four => {
                array_vec!([Poly<Normalised>; 4] => Poly::new(), Poly::new(), Poly::new(), Poly::new())
            }
        };

        Self {
            polynomials,
            sec_level: k,
        }
    }

    fn ntt(&self) -> PolyVec<Unnormalised> {
        let mut polynomials = ArrayVec::<[Poly<Unnormalised>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.ntt());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }

    fn inv_ntt(&self) -> Self {
        let mut polynomials = ArrayVec::<[Poly<Normalised>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.inv_ntt());
        }

        Self {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}


struct Matrix<S: State> {
    vectors: ArrayVec<[PolyVec<S>; 4]>,
    sec_level: K,
}


// trait SameSecLevel {}

// pub trait PolyVecOperations {
//     fn new_filled() -> Self;
//     fn add(&mut self, addend: Self);
//     fn barrett_reduce(&mut self);
//     fn normalise(&mut self);
//     fn ntt(&mut self);
//     fn inv_ntt(&mut self);
//     fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta);
//     fn pack(&self, buf: &mut [u8]);
//     fn unpack(&mut self, buf: &[u8]);
//     fn compress(&self, buf: &mut [u8]) -> Result<(), TryFromIntError>;
//     fn decompress(&mut self, buf: &[u8]) -> Result<(), TryFromIntError>;
// }

// macro_rules! impl_polyvec {
//     ($variant:ty) => {
//         impl PolyVecOperations for $variant {
//             fn new_filled() -> Self {
//                 let mut poly_vec = Self::default();
//                 for _ in 0..poly_vec.capacity() {
//                     poly_vec.push(Poly::new());
//                 }
//                 poly_vec
//             }

//             fn add(&mut self, addend: Self) {
//                 for (augend_poly, addend_poly) in self.iter_mut().zip(addend.iter()) {
//                     augend_poly.add(&addend_poly);
//                 }
//             }

//             fn barrett_reduce(&mut self) {
//                 for poly in self.iter_mut() {
//                     poly.barrett_reduce();
//                 }
//             }

//             fn normalise(&mut self) {
//                 for poly in self.iter_mut() {
//                     poly.normalise();
//                 }
//             }

//             fn ntt(&mut self) {
//                 for poly in self.iter_mut() {
//                     poly.ntt();
//                 }
//             }

//             fn inv_ntt(&mut self) {
//                 for poly in self.iter_mut() {
//                     poly.inv_ntt();
//                 }
//             }

//             fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: Eta) {
//                 for poly in self.iter_mut() {
//                     poly.derive_noise(seed, nonce, eta);
//                 }
//             }

//             // buf should be of length K * POLYBYTES
//             fn pack(&self, buf: &mut [u8]) {
//                 for (k, poly) in self.iter().enumerate() {
//                     poly.pack(&mut buf[k * POLYBYTES..(k + 1) * POLYBYTES]);
//                 }
//             }

//             fn unpack(&mut self, buf: &[u8]) {
//                 for (k, poly) in self.iter_mut().enumerate() {
//                     poly.unpack(&buf[k * POLYBYTES..(k + 1) * POLYBYTES]);
//                 }
//             }

//             // buf should be of length poly_vec_compressed_bytes
//             fn compress(&self, buf: &mut [u8]) -> Result<(), TryFromIntError> {
//                 let k_value: u8 = <$variant as GetSecLevel>::sec_level().k().into();

//                 match <$variant as GetSecLevel>::sec_level() {
//                     SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
//                         for i in 0..usize::from(k_value) {
//                             for j in 0..N / 4 {
//                                 let mut temp = [0u16; 4];

//                                 for k in 0..4 {
//                                     temp[k] = u16::try_from(self[i].coeffs[4 * j + k])?;
//                                     temp[k] = temp[k].wrapping_add(u16::try_from(
//                                         (i16::try_from(temp[k])? >> 15) & i16::try_from(Q)?,
//                                     )?);
//                                     temp[k] = u16::try_from(
//                                         (((u32::from(temp[k]) << 10) + u32::try_from(Q)? / 2)
//                                             / u32::try_from(Q)?)
//                                             & 0x3ff,
//                                     )?;
//                                 }

//                                 let index = (i * (N / 4) + j) * 5;

//                                 buf[index..index + 5].copy_from_slice(&[
//                                     temp[0] as u8,
//                                     ((temp[0] >> 8) | (temp[1] << 2)) as u8,
//                                     ((temp[1] >> 6) | (temp[2] << 4)) as u8,
//                                     ((temp[2] >> 4) | (temp[3] << 6)) as u8,
//                                     (temp[3] >> 2) as u8,
//                                 ]);
//                             }
//                         }
//                     }
//                     SecurityLevel::TenTwoFour { .. } => {
//                         for i in 0..usize::from(k_value) {
//                             for j in 0..N / 8 {
//                                 let mut temp = [0u16; 8];

//                                 for k in 0..8 {
//                                     temp[k] = u16::try_from(self[i].coeffs[8 * j + k])?;
//                                     temp[k] = temp[k].wrapping_add(u16::try_from(
//                                         (i16::try_from(temp[k])? >> 15) & i16::try_from(Q)?,
//                                     )?);
//                                     temp[k] = u16::try_from(
//                                         (((u32::from(temp[k]) << 11) + u32::try_from(Q)? / 2)
//                                             / u32::try_from(Q)?)
//                                             & 0x7ff,
//                                     )?;
//                                 }

//                                 let index = (i * (N / 8) + j) * 11;

//                                 buf[index..index + 11].copy_from_slice(&[
//                                     temp[0] as u8,
//                                     ((temp[0] >> 8) | (temp[1] << 3)) as u8,
//                                     ((temp[1] >> 5) | (temp[2] << 6)) as u8,
//                                     (temp[2] >> 2) as u8,
//                                     ((temp[2] >> 10) | (temp[3] << 1)) as u8,
//                                     ((temp[3] >> 7) | (temp[4] << 4)) as u8,
//                                     ((temp[4] >> 4) | (temp[5] << 7)) as u8,
//                                     (temp[5] >> 1) as u8,
//                                     ((temp[5] >> 9) | (temp[6] << 2)) as u8,
//                                     ((temp[6] >> 6) | (temp[7] << 5)) as u8,
//                                     (temp[7] >> 3) as u8,
//                                 ]);
//                             }
//                         }
//                     }
//                 }
//                 Ok(())
//             }

//             // buf should be of length poly_vec_compressed_bytes
//             fn decompress(&mut self, buf: &[u8]) -> Result<(), TryFromIntError> {
//                 let k_value: u8 = <$variant as GetSecLevel>::sec_level().k().into();

//                 match <$variant as GetSecLevel>::sec_level() {
//                     SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
//                         for i in 0..usize::from(k_value) {
//                             for j in 0..N / 4 {
//                                 let index = (i * (N / 4) + j) * 5;

//                                 let temp = (0..4).map(|k| {
//                                     let shift = (2 * k) as u32;
//                                     let val = u16::from(buf[index + k] >> shift)
//                                         | u16::from(buf[index + k + 1]) << (8 - shift);
//                                     val
//                                 });

//                                 for (k, val) in temp.enumerate() {
//                                     self[i].coeffs[4 * j + k] = i16::try_from(
//                                         ((u32::from(val) & 0x3ff) * u32::try_from(Q)? + 512) >> 10,
//                                     )?;
//                                 }
//                             }
//                         }
//                     }
//                     SecurityLevel::TenTwoFour { .. } => {
//                         for i in 0..usize::from(k_value) {
//                             for j in 0..N / 8 {
//                                 let mut index = (i * (N / 8) + j) * 11;

//                                 let temp = (0..8).map(|k| {
//                                     let shift = ((3 * k) % 8) as u32;
//                                     let mut val = u16::from(buf[index + k] >> shift)
//                                         | u16::from(buf[index + k + 1]) << (8 - shift);
//                                     if k % 3 == 2 {
//                                         let mut extra = u16::from(buf[index + k + 2]);
//                                         if k == 2 {
//                                             extra <<= 10;
//                                         } else if k == 5 {
//                                             extra <<= 9;
//                                         }
//                                         val |= extra;
//                                         index += 1;
//                                     }
//                                     val
//                                 });

//                                 for (k, val) in temp.enumerate() {
//                                     self[i].coeffs[8 * j + k] = i16::try_from(
//                                         (u32::from(val & 0x7ff) * u32::try_from(Q)? + 1024) >> 11,
//                                     )?;
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 Ok(())
//             }
//         }
//         impl SameSecLevel for $variant {}
//     };
// }

// impl_polyvec!(PolyVec512);
// impl_polyvec!(PolyVec768);
// impl_polyvec!(PolyVec1024);

// pub trait LinkSecLevel<P: PolyVecOperations> {}
// impl LinkSecLevel<PolyVec512> for Mat512 {}
// impl LinkSecLevel<PolyVec768> for Mat768 {}
// impl LinkSecLevel<PolyVec1024> for Mat1024 {}

// impl Poly {
//     pub fn inner_product_pointwise<T>(&mut self, multiplicand: T, multiplier: T)
//     where
//         T: PolyVecOperations + IntoIterator<Item = Self>,
//     {
//         *self = Self::new(); // Zero output Poly
//         for (multiplicand_poly, multiplier_poly) in
//             multiplicand.into_iter().zip(multiplier.into_iter())
//         {
//             let mut temp = multiplicand_poly;
//             temp.pointwise_mul(&multiplier_poly);
//             self.add(&temp);
//         }
//         self.barrett_reduce();
//     }
// }
