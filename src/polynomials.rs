use crate::{
    field_operations::{barrett_reduce, conditional_sub_q, mont_form, montgomery_reduce},
    ntt::ZETAS,
    params::{SecurityLevel, N, Q, SYMBYTES, POLYBYTES},
};
use core::num::TryFromIntError;
use tinyvec::ArrayVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Poly<S: State> {
    pub(crate) coeffs: [i16; N],
    state: S,
}

struct Normalised;
struct Unnormalised;

pub trait State {}
impl State for Normalised {}
impl State for Unnormalised {}

impl Default for Poly<Normalised> {
    fn default() -> Self {
        Self {
            coeffs: [0; N],
            state: Normalised,
        }
    }
}

impl<S: State> Poly<S> {
    /// Sets self to self + x
    /// Example:
    /// ```
    /// let new_poly = poly1.add(&poly2);
    /// ```
    fn add(&self, x: &Self) -> Poly<Unnormalised> {
        let coeffs_arr: [i16; N] = self.coeffs.iter().zip(x.coeffs.iter())
            .map(|(&a, &b)| a + b)
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unnormalised,
        }
    }

    /// Sets self to self - x
    /// Example:
    /// ```
    /// let new_poly = poly1.sub(&poly2);
    /// ```
    pub(crate) fn sub(&self, x: &Self) -> Poly<Unnormalised> {
        let coeffs_arr: [i16; N] = self.coeffs.iter().zip(x.coeffs.iter())
            .map(|(&a, &b)| a - b)
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unnormalised,
        }
    }

    /// Barrett reduces all coefficients of given polynomial
    /// Coefficients are nearly normalise, lying within {0..q}
    /// Example:
    /// ```
    /// let reduced_poly = poly.barrett_reduce();
    /// ```
    pub(crate) fn barrett_reduce(&self) -> Poly<Unnormalised> {
        let coeffs_arr: [i16; N] = self.coeffs.iter()
            .map(|&coeff| barrett_reduce(coeff))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unnormalised,
        }
    }

    /// Converts all coefficients of the given polynomial to Mongomery form
    /// All coefficients are bounded in absolute value by q.
    /// Example:
    /// ```
    /// let reduced_poly = poly.mont_form();
    /// ```
    pub(crate) fn mont_form(&self) -> Poly<Unnormalised> {
        let coeffs_arr: [i16; N] = self.coeffs.iter()
            .map(|&coeff| mont_form(coeff))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unnormalised,
        }
    }    

    /// Pointwise multiplication of two polynomials,
    /// If the inputs are of montgomery form, then so will the output, bounded by 2q.
    /// If the inputs are not of montgomery form, then the output will also be unnormalised.
    /// Products of coefficients of the two polynomials must be strictly bound by 2^15 q.
    /// Example:
    /// ```
    /// let new_poly = poly1.pointwise_mul(&poly2);
    pub(crate) fn pointwise_mul(&self, x: &Self) -> Poly<Unnormalised> {
        let mut coeffs_arr = self.coeffs;
        for ((chunk, x_chunk), &zeta) in coeffs_arr
            .chunks_mut(4)
            .zip(x.coeffs.chunks(4))
            .zip(ZETAS.iter().skip(64))
        {
            let mut temp = [0i16; 4];

            for (i, coeff) in temp.iter_mut().enumerate() {
                if i % 2 == 0 {
                    let sign: i16 = if i == 2 { -1 } else { 1 };
                    *coeff = montgomery_reduce(i32::from(chunk[i + 1]) * i32::from(x_chunk[i + 1]));
                    *coeff = sign * montgomery_reduce(i32::from(*coeff) * i32::from(zeta));
                    *coeff += montgomery_reduce(i32::from(chunk[i]) * i32::from(x_chunk[i]));
                } else {
                    *coeff = montgomery_reduce(i32::from(chunk[i - 1]) * i32::from(x_chunk[i]));
                    *coeff += montgomery_reduce(i32::from(chunk[i]) * i32::from(x_chunk[i - 1]));
                }
            }
            chunk.copy_from_slice(&temp);
        }
        Poly {
            coeffs: coeffs_arr,
            state: Unnormalised,
        }
    }
}


impl Poly<Unnormalised> {
    /// Normalise coefficients of given polynomial
    /// Normalised coefficients lie within {0..q-1}
    /// Example:
    /// ```
    /// let new_poly = poly.normalise();
    /// ```
    fn normalise(&self) -> Poly<Normalised>{
        let coeffs_arr: [i16; N] = self.coeffs.iter()
            .map(|&coeff| conditional_sub_q(barrett_reduce(coeff)))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Normalised,
        }
    }
}


impl Poly<Normalised> {
    /// const function equivelent of default (default is needed for ArrayVec)
    /// Example:
    /// ```
    /// let poly = Poly::new();
    /// ```
    const fn new() -> Poly<Normalised> {
        Poly {
            coeffs: [0; N],
            state: Normalised,
        }
    }

    /// Creates a poly from a given array slice.
    /// Output is Unnormalised as we do not know whether the input array is normalised
    /// Example:
    /// ```
    /// let poly = Poly::from(&[1i16; N]);
    const fn from(array: &[i16; N]) -> Poly<Unnormalised> {
        Poly {
            coeffs: *array,
            state: Unnormalised,
        }
    }

    /// Packs given poly into a 384-byte (POLYBYTES size) buffer
    /// must be normalised
    /// Example:
    /// ```
    /// let buf = poly.pack();
    /// ```
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    pub(crate) fn pack(&self) -> [u8; POLYBYTES] {
        let mut buf = [0u8; POLYBYTES];
        for i in 0..N / 2 {
            let mut t0 = self.coeffs[2 * i];
            t0 += (t0 >> 15) & Q as i16;
            let mut t1 = self.coeffs[2 * i + 1];
            t1 += (t1 >> 15) & Q as i16;

            buf[3 * i] = t0 as u8;
            buf[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            buf[3 * i + 2] = (t1 >> 4) as u8;
        }

        buf
    }
    
    /// Convert a given polynomial into a SYMBYTES (32-byte) message
    /// poly should be normalised
    /// Example:
    /// ```
    /// let msg_result = match poly.write_msg()?;
    /// ```
    pub(crate) fn write_msg(&self) -> Result<[u8; SYMBYTES], TryFromIntError> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let q_16 = i16::try_from(Q)?;
        let buf = self.coeffs.chunks_exact(8)
            .map(|chunk|chunk.iter()
                .map(|&coeff| coeff + ((coeff >> 15) & q_16))
                .map(|&coeff| (((coeff << 1) + q_16 / 2) / q_16) & 1)
                .enumerate()
                .try_fold(0, |accumulator, (index, &coeff)| {
                    let shifted_coeff = u8::try_from(coeff << index)?;
                    Ok(accumulator | shifted_coeff)
                })
            )
            .collect::<Result<ArrayVec<[u8; SYMBYTES]>, TryFromIntError>>();
            
        buf
    }

    /// Compress polynomial to a buffer
    /// buf must have space for poly_compressed_bytes
    /// poly should be normalised
    /// Example:
    /// ```
    /// poly.compress(buf, sec_level)?;
    /// ```
    pub(crate) fn compress(
        &self,
        buf: &mut [u8],
        sec_level: &SecurityLevel,
    ) -> Result<(), TryFromIntError> {
        let mut t = [0u8; 8];

        match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                for (coeff_chunk, buf_chunk) in self.coeffs.chunks_exact(8)
                    .zip(buf.chunks_exact_mut(4)) {
                    for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                        let mut temp = *coeff;
                        temp += (temp >> 15) & i16::try_from(Q)?;
                        *t_elem = u8::try_from(
                            ((((u16::try_from(temp)?) << 4) + u16::try_from(Q)? / 2)
                                / u16::try_from(Q)?)
                                & 15,
                        )?;
                    }

                    buf_chunk.copy_from_slice(
                        &t.chunks_exact(2)
                        .map(|chunk| chunk[0] | (chunk[1] << 4))
                        .collect::<ArrayVec<[i16; N]>>()
                        .into_inner()
                    );
                }
                Ok(())
            }
            SecurityLevel::TenTwoFour { .. } => {
                for (coeff_chunk, buf_chunk) in self.coeffs.chunks_exact(8)
                    .zip(buf.chunks_exact_mut(5)) {
                    for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                        let mut temp = *coeff;
                        temp += (temp >> 15) & i16::try_from(Q)?;
                        *t_elem = u8::try_from(
                            ((((u32::try_from(temp)?) << 5) + u32::try_from(Q)? / 2)
                                / u32::try_from(Q)?)
                                & 31,
                        )?;
                    }

                    buf_chunk.copy_from_slice(&[
                        t[0] | (t[1] << 5),
                        (t[1] >> 3) | (t[2] << 2) | (t[3] << 7),
                        (t[3] >> 1) | (t[4] << 4),
                        (t[4] >> 4) | (t[5] << 1) | (t[6] << 6),
                        (t[6] >> 2) | (t[7] << 3),
                    ]);
                }
                Ok(())
            }
        }
    }
}


/// Unpacks a buffer of POLYBYTES bytes into a polynomial
/// poly will NOT be normalised, but 0 <= coeffs < 4096
/// Example:
/// poly.unpack(buf);
fn unpack_to_poly(buf: &[u8]) -> Poly<Unnormalised>{
    let coeffs_arr: [i16; N] = buf.chunks_exact(3)
        .flat_map(|chunk| chunk.windows(2).enumerate())
        .map(|(index, pair)| {
            if index % 2 == 0 {
                i16::from(pair[0]) | ((i16::from(pair[1]) << 8) & 0xfff)
            } else {
                i16::from(pair[0] >> 4) | ((i16::from(pair[1]) << 4) & 0xfff)
            }
        }).collect::<ArrayVec<[i16; N]>>()
        .into_inner();                    

    Poly {
        coeffs: coeffs_arr,
        state: Unnormalised,
    }
}

/// Converts a message buffer into a polynomial
/// msg should be of length SYMBYTES (32)
/// poly will not be normalised
/// Example:
/// poly.read_msg(msg_buf);
fn read_msg_to_poly(msg: &[u8]) -> Result<Poly<Unnormalised>, TryFromIntError> {
    let q_plus_one_over_2 = i16::try_from((Q + 1) / 2)?;
    let coeffs_arr: [i16; N] = msg.iter()
        .flat_map(|&byte| (0..8).map(move |i| ((i16::from(byte) >> i) & 1).wrapping_neg()))
        .map(|mask| mask & q_plus_one_over_2)
        .collect::<ArrayVec<[i16; N]>>()
        .into_inner();

    Ok(Poly {
        coeffs: coeffs_arr,
        state: Unnormalised,
    })
}

/// Decompresses buffer into a polynomial
/// is dependent on the security level
/// buf should be of length poly_compressed_bytes
/// output poly is normalised
/// Example:
/// poly.decompress(buf, k);
fn decompress_to_poly(buf: &[u8], sec_level: &SecurityLevel) -> Result<Poly<Normalised>, TryFromIntError> {
    match sec_level {
        SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
            let coeffs_arr: [i16; N] = buf.iter()
                .flat_map(|&byte| (0..2).map(move |i| {
                    if i == 0 {
                        (usize::from(byte & 15) * Q + 8) >> 4
                    } else {
                        (usize::from(byte >> 4) * Q + 8) >> 4
                    }
                }))
                .map(i16::try_from)
                .collect::<Result<ArrayVec<[i16; N]>, TryFromIntError>>()?
                .into_inner();

            Ok(Poly {
                coeffs: coeffs_arr,
                state: Normalised,
            })
        }
        SecurityLevel::TenTwoFour { .. } => {
            let mut coeffs_arr = [0i16; N];
            for (coeffs_chunk, buf_chunk) in coeffs_arr.chunks_exact_mut(8)
                .zip(buf.chunks_exact(5)) {
                let temp: [u8; 8] = [
                    buf_chunk[0],
                    (buf_chunk[0] >> 5) | (buf_chunk[1] << 3),
                    buf_chunk[1] >> 2,
                    (buf_chunk[1] >> 7) | (buf_chunk[2] << 1),
                    (buf_chunk[2] >> 4) | (buf_chunk[3] << 4),
                    buf_chunk[3] >> 1,
                    (buf_chunk[3] >> 6) | (buf_chunk[4] << 2),
                    buf_chunk[4] >> 3
                ];
                for (coeff, t_elem) in coeffs_chunk.iter_mut().zip(temp.iter()) {
                    *coeff = i16::try_from(((u32::from(*t_elem) & 31) * u32::try_from(Q)? + 16) >> 5)?;
                }
            }
            Ok(Poly {
                coeffs: coeffs_arr,
                state: Normalised
            })
        }
    }
}


// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub struct Poly {
//     pub(crate) coeffs: [i16; N],
// }

// impl Default for Poly {
//     fn default() -> Self {
//         Self { coeffs: [0; N] }
//     }
// }

// impl Poly {
//     // const function equivelent of default (default is needed for ArrayVec)
//     // Example:
//     // let poly = Poly::new();
//     pub(crate) const fn new() -> Self {
//         Self { coeffs: [0; N] }
//     }

//     pub(crate) const fn from(array: &[i16; N]) -> Self {
//         Self { coeffs: *array }
//     }

//     // Sets self to self + x
//     // Example:
//     // poly1.add(&poly2);
//     pub(crate) fn add(&mut self, x: &Self) {
//         for i in 0..N {
//             self.coeffs[i] += x.coeffs[i];
//         }
//     }

//     // Sets self to self - x
//     // Example:
//     // poly1.sub(&poly2);
//     pub(crate) fn sub(&mut self, x: &Self) {
//         for i in 0..N {
//             self.coeffs[i] -= x.coeffs[i];
//         }
//     }

//     // Normalise coefficients of given polynomial
//     // Example:
//     // poly.normalise();
//     pub(crate) fn normalise(&mut self) {
//         for coeff in &mut self.coeffs {
//             *coeff = conditional_sub_q(barrett_reduce(*coeff));
//         }
//     }

//     // Barrett reduces all coefficients of given polynomial
//     // Example:
//     // poly.barrett_reduce();
//     pub(crate) fn barrett_reduce(&mut self) {
//         for coeff in &mut self.coeffs {
//             *coeff = barrett_reduce(*coeff);
//         }
//     }

//     // Converts all coefficients of the given polynomial to Mongomery form
//     // Example:
//     // poly.mont_form();
//     pub(crate) fn mont_form(&mut self) {
//         for coeff in &mut self.coeffs {
//             *coeff = mont_form(*coeff);
//         }
//     }

//     // Pointwise multiplication of two polynomials,
//     // assumes inputs are of montgomery form.
//     // Example:
//     // poly1.pointwise_mul(&poly2);
//     pub(crate) fn pointwise_mul(&mut self, x: &Self) {
//         for ((chunk, x_chunk), &zeta) in self
//             .coeffs
//             .chunks_mut(4)
//             .zip(x.coeffs.chunks(4))
//             .zip(ZETAS.iter().skip(64))
//         {
//             let mut temp = [0i16; 4];

//             for (i, coeff) in temp.iter_mut().enumerate() {
//                 if i % 2 == 0 {
//                     let sign: i16 = if i == 2 { -1 } else { 1 };
//                     *coeff = montgomery_reduce(i32::from(chunk[i + 1]) * i32::from(x_chunk[i + 1]));
//                     *coeff = sign * montgomery_reduce(i32::from(*coeff) * i32::from(zeta));
//                     *coeff += montgomery_reduce(i32::from(chunk[i]) * i32::from(x_chunk[i]));
//                 } else {
//                     *coeff = montgomery_reduce(i32::from(chunk[i - 1]) * i32::from(x_chunk[i]));
//                     *coeff += montgomery_reduce(i32::from(chunk[i]) * i32::from(x_chunk[i - 1]));
//                 }
//             }
//             chunk.copy_from_slice(&temp);
//         }
//     }

//     // Packs given poly into a 384-byte (POLYBYTES size) buffer
//     // must be normalised
//     // Example:
//     // poly.pack(buf);
//     #[allow(
//         clippy::cast_possible_truncation,
//         clippy::cast_sign_loss,
//         clippy::cast_possible_wrap
//     )]
//     pub(crate) fn pack(&self, buf: &mut [u8]) {
//         for i in 0..N / 2 {
//             let mut t0 = self.coeffs[2 * i];
//             t0 += (t0 >> 15) & Q as i16;
//             let mut t1 = self.coeffs[2 * i + 1];
//             t1 += (t1 >> 15) & Q as i16;

//             buf[3 * i] = t0 as u8;
//             buf[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
//             buf[3 * i + 2] = (t1 >> 4) as u8;
//         }
//     }

//     // Unpacks a buffer of bytes into a polynomial
//     // poly will NOT be normalised, but 0 <= coeffs < 4096
//     // Example:
//     // poly.unpack(buf);
//     pub(crate) fn unpack(&mut self, buf: &[u8]) {
//         for i in 0..N / 2 {
//             self.coeffs[2 * i] = i16::from(buf[3 * i]) | ((i16::from(buf[3 * i + 1]) << 8) & 0xfff);
//             self.coeffs[2 * i + 1] =
//                 i16::from(buf[3 * i + 1] >> 4) | ((i16::from(buf[3 * i + 2]) << 4) & 0xfff);
//         }
//     }

//     // Converts a message buffer into a polynomial
//     // msg should be of length SYMBYTES (32)
//     // poly will not be normalised
//     // Example:
//     // poly.read_msg(msg_buf);
//     pub(crate) fn read_msg(&mut self, msg: &[u8]) -> Result<(), TryFromIntError> {
//         for i in 0..SYMBYTES {
//             for j in 0..8 {
//                 let mask = ((i16::from(msg[i]) >> j) & 1).wrapping_neg();
//                 self.coeffs[8 * i + j] = mask & i16::try_from((Q + 1) / 2)?;
//             }
//         }
//         Ok(())
//     }

//     // Convert a given polynomial into a SYMBYTES (32-byte) message
//     // poly should be normalised
//     // Example:
//     // poly.write_msg(msg_buf);
//     pub(crate) fn write_msg(&self, buf: &mut [u8]) -> Result<(), TryFromIntError> {
//         #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
//         let q_16 = i16::try_from(Q)?;
//         for i in 0..N / 8 {
//             buf[i] = 0;
//             for j in 0..8 {
//                 let mut x = self.coeffs[8 * i + j];
//                 x += (x >> 15) & q_16;
//                 x = (((x << 1) + q_16 / 2) / q_16) & 1;
//                 buf[i] |= u8::try_from(x << j)?;
//             }
//         }
//         Ok(())
//     }

//     // Decompresses buffer into a polynomial
//     // is dependent on the security level
//     // buf should be of length poly_compressed_bytes
//     // output poly is normalised
//     // Example:
//     // poly.decompress(buf, k);
//     pub(crate) fn decompress(
//         &mut self,
//         buf: &[u8],
//         sec_level: &SecurityLevel,
//     ) -> Result<(), TryFromIntError> {
//         let mut k = 0usize;

//         match sec_level {
//             SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
//                 for (i, &byte) in buf.iter().take(N / 2).enumerate() {
//                     self.coeffs[2 * i] = i16::try_from((usize::from(byte & 15) * Q + 8) >> 4)?;
//                     self.coeffs[2 * i + 1] = i16::try_from((usize::from(byte >> 4) * Q + 8) >> 4)?;
//                 }
//                 Ok(())
//             }
//             SecurityLevel::TenTwoFour { .. } => {
//                 let mut t = [0u8; 8];
//                 for i in 0..N / 8 {
//                     t[0] = buf[k];
//                     t[1] = (buf[k] >> 5) | (buf[k + 1] << 3);
//                     t[2] = buf[k + 1] >> 2;
//                     t[3] = (buf[k + 1] >> 7) | (buf[k + 2] << 1);
//                     t[4] = (buf[k + 2] >> 4) | (buf[k + 3] << 4);
//                     t[5] = buf[k + 3] >> 1;
//                     t[6] = (buf[k + 3] >> 6) | (buf[k + 4] << 2);
//                     t[7] = buf[k + 4] >> 3;
//                     k += 5;

//                     for (j, t_elem) in t.iter().enumerate() {
//                         self.coeffs[8 * i + j] = i16::try_from(
//                             ((u32::from(*t_elem) & 31) * u32::try_from(Q)? + 16) >> 5,
//                         )?;
//                     }
//                 }
//                 Ok(())
//             }
//         }
//     }

//     // Compress polynomial to a buffer
//     // buf must have space for poly_compressed_bytes
//     // poly should be normalised
//     // Example:
//     // self.compress(buf);
//     pub(crate) fn compress(
//         &self,
//         buf: &mut [u8],
//         sec_level: &SecurityLevel,
//     ) -> Result<(), TryFromIntError> {
//         let mut k = 0usize;
//         let mut t = [0u8; 8];

//         match sec_level {
//             SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
//                 for i in 0..N / 8 {
//                     for j in 0..8 {
//                         let mut u = self.coeffs[8 * i + j];
//                         u += (u >> 15) & i16::try_from(Q)?;
//                         t[j] = u8::try_from(
//                             ((((u16::try_from(u)?) << 4) + u16::try_from(Q)? / 2)
//                                 / u16::try_from(Q)?)
//                                 & 15,
//                         )?;
//                     }

//                     buf[k..k + 4].copy_from_slice(&[
//                         t[0] | (t[1] << 4),
//                         t[2] | (t[3] << 4),
//                         t[4] | (t[5] << 4),
//                         t[6] | (t[7] << 4),
//                     ]);
//                     k += 4;
//                 }
//                 Ok(())
//             }
//             SecurityLevel::TenTwoFour { .. } => {
//                 for i in 0..N / 8 {
//                     for j in 0..8 {
//                         let mut u = self.coeffs[8 * i + j];
//                         u += (u >> 15) & i16::try_from(Q)?;
//                         t[j] = u8::try_from(
//                             ((((u32::try_from(u)?) << 5) + u32::try_from(Q)? / 2)
//                                 / u32::try_from(Q)?)
//                                 & 31,
//                         )?;
//                     }

//                     buf[k..k + 5].copy_from_slice(&[
//                         t[0] | (t[1] << 5),
//                         (t[1] >> 3) | (t[2] << 2) | (t[3] << 7),
//                         (t[3] >> 1) | (t[4] << 4),
//                         (t[4] >> 4) | (t[5] << 1) | (t[6] << 6),
//                         (t[6] >> 2) | (t[7] << 3),
//                     ]);
//                     k += 5;
//                 }
//                 Ok(())
//             }
//         }
//     }
// }
