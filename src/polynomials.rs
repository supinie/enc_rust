mod ntt;
mod sample;

use crate::{
    errors::{CrystalsError, PackingError},
    field_operations::{barrett_reduce, conditional_sub_q, mont_form, montgomery_reduce},
    params::{SecurityLevel, N, POLYBYTES, Q, Q_DIV, Q_I16, Q_U16, Q_I32, Q_U32, SYMBYTES},
    polynomials::ntt::ZETAS,
};
use core::num::TryFromIntError;
use tinyvec::ArrayVec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Poly<S: State> {
    coeffs: [i16; N],
    state: S,
}

// Normalised coefficients lie within {0..q-1}
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Normalised;
// Barrett reduced (almost normal) coefficients lie within {0..q}
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Barrett;
// Montogomery form coefficients lie within {-q..q}
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Montgomery;
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Unreduced;

pub trait State: Default {}
impl State for Normalised {}
impl State for Barrett {}
impl State for Montgomery {}
impl State for Unreduced {}

pub trait Unnormalised: Default {}
impl Unnormalised for Barrett {}
impl Unnormalised for Montgomery {}
impl Unnormalised for Unreduced {}

pub trait Reduced: Default {}
impl Reduced for Normalised {}
impl Reduced for Barrett {}
impl Reduced for Montgomery {}

// In all cases, `new()` should be used instead, else the state may be incorrect.
// Default is defined here for `ArrayVec`.
impl<S: State> Default for Poly<S> {
    fn default() -> Self {
        Self {
            coeffs: [0; N],
            state: Default::default(),
        }
    }
}

impl<S: State> Poly<S> {
    pub(crate) const fn coeffs(&self) -> &[i16; N] {
        &self.coeffs
    }

    // Sets self to self + x
    // The coeffs of self and x should be small enough that no overflow can occur.
    // If in doubt, reduce first.
    // Example:
    // ```
    // let new_poly = poly1.add(&poly2);
    // ```
    pub(crate) fn add<T: State>(&self, x: &Poly<T>) -> Poly<Unreduced> {
        let coeffs_arr: [i16; N] = self
            .coeffs
            .iter()
            .zip(x.coeffs.iter())
            .map(|(&a, &b)| a + b)
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unreduced,
        }
    }

    // Sets self to self - x
    // Example:
    // ```
    // let new_poly = poly1.sub(&poly2);
    // ```
    pub(crate) fn sub<T: State>(&self, x: &Poly<T>) -> Poly<Unreduced> {
        let coeffs_arr: [i16; N] = self
            .coeffs
            .iter()
            .zip(x.coeffs.iter())
            .map(|(&a, &b)| a - b)
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Unreduced,
        }
    }

    // Barrett reduces all coefficients of given polynomial
    // Coefficients are nearly normalise, lying within {0..q}
    // Example:
    // ```
    // let reduced_poly = poly.barrett_reduce();
    // ```
    pub(crate) fn barrett_reduce(&self) -> Poly<Barrett> {
        let coeffs_arr: [i16; N] = self
            .coeffs
            .iter()
            .map(|&coeff| barrett_reduce(coeff))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Barrett,
        }
    }

    // Converts all coefficients of the given polynomial to Mongomery form
    // All coefficients are bounded in absolute value by q.
    // Example:
    // ```
    // let reduced_poly = poly.mont_form();
    // ```
    pub(crate) fn mont_form(&self) -> Poly<Montgomery> {
        let coeffs_arr: [i16; N] = self
            .coeffs
            .iter()
            .map(|&coeff| mont_form(coeff))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Montgomery,
        }
    }
}

impl<S: State + Unnormalised> Poly<S> {
    // Normalise coefficients of given polynomial
    // Normalised coefficients lie within {0..q-1}
    // Example:
    // ```
    // let normal_poly = poly.normalise();
    // ```
    pub(crate) fn normalise(&self) -> Poly<Normalised> {
        let coeffs_arr: [i16; N] = self
            .coeffs
            .iter()
            .map(|&coeff| conditional_sub_q(barrett_reduce(coeff)))
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();
        Poly {
            coeffs: coeffs_arr,
            state: Normalised,
        }
    }
}

impl<S: State + Reduced> Poly<S> {
    // Pointwise multiplication of two polynomials,
    // If the inputs are of montgomery form, then so will the output, bounded by 2q.
    // If the inputs are not of montgomery form, then the output will also be unnormalised.
    // Products of coefficients of the two polynomials must be strictly bound by 2^15 q.
    // Example:
    // ```
    // let new_poly = poly1.pointwise_mul(&poly2);
    // ```
    pub(crate) fn pointwise_mul<T: State>(&self, x: &Poly<T>) -> Poly<Unreduced> {
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
            state: Unreduced,
        }
    }
}

impl Poly<Normalised> {
    // const function equivelent of `default` (`default` is needed for `ArrayVec`)
    // Example:
    // ```
    // let poly = Poly::new();
    // ```
    pub const fn new() -> Self {
        Self {
            coeffs: [0; N],
            state: Normalised,
        }
    }

    // Creates a poly from a given array slice.
    // Output is Unnormalised as we do not know whether the input array is normalised
    // Example:
    // ```
    // let poly = Poly::from(&[1i16; N]);
    // ```
    pub(crate) const fn from_arr(array: &[i16; N]) -> Poly<Unreduced> {
        Poly {
            coeffs: *array,
            state: Unreduced,
        }
    }

    // USE WITH CAUTION
    pub(crate) const fn from_arr_normal(array: &[i16; N]) -> Self {
        Self {
            coeffs: *array,
            state: Normalised,
        }
    }

    // Packs given poly into a 384-byte (POLYBYTES size) buffer
    // Poly must be normalised
    // Example:
    // ```
    // let buf = poly.pack();
    // ```
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    pub(crate) fn pack(&self) -> [u8; POLYBYTES] {
        let mut buf = [0u8; POLYBYTES];
        for i in 0..N / 2 {
            let mut t0 = self.coeffs[2 * i];
            t0 += (t0 >> 15) & Q_I16;
            let mut t1 = self.coeffs[2 * i + 1];
            t1 += (t1 >> 15) & Q_I16;

            buf[3 * i] = t0 as u8;
            buf[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            buf[3 * i + 2] = (t1 >> 4) as u8;
        }

        buf
    }

    // Convert a given polynomial into a SYMBYTES (32-byte) message
    // poly should be normalised
    // Example:
    // ```
    // let msg_result = poly.write_msg()?;
    // ```
    pub(crate) fn write_msg(&self) -> Result<[u8; SYMBYTES], TryFromIntError> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let buf = self
            .coeffs
            .chunks_exact(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&coeff| coeff + ((coeff >> 15) & Q_I16))
                    .map(i32::from)
                    .map(|coeff| (coeff << 1) + Q_I32 / 2)
                    .map(|t| ((t * 80635) >> 28) & 1)
                    .enumerate()
                    .try_fold(0, |accumulator, (index, coeff)| {
                        let shifted_coeff = u8::try_from(coeff << index)?;
                        Ok(accumulator | shifted_coeff)
                    })
            })
            .collect::<Result<ArrayVec<[u8; SYMBYTES]>, TryFromIntError>>()
            .map(ArrayVec::into_inner);

        buf
    }

    // Compress polynomial to a buffer
    // buf must have space for `poly_compressed_bytes`
    // poly should be normalised
    // Example:
    // ```
    // my_poly.compress(&buf, sec_level)?;
    // ```
    pub(crate) fn compress(
        &self,
        buf: &mut [u8],
        sec_level: &SecurityLevel,
    ) -> Result<(), PackingError> {
        let mut t = [0u8; 8];

        if buf.len() != sec_level.poly_compressed_bytes() {
            return Err(CrystalsError::IncorrectBufferLength(
                buf.len(),
                sec_level.poly_compressed_bytes(),
            )
            .into());
        }
        match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                for (coeff_chunk, buf_chunk) in
                    self.coeffs.chunks_exact(8).zip(buf.chunks_exact_mut(4))
                {
                    for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                        let mut temp = *coeff;
                        temp += (temp >> 15) & Q_I16;
                        *t_elem = u8::try_from(
                            (((((u64::try_from(temp)?) << 4) + u64::from(Q_U16 / 2)) * Q_DIV)
                                >> 28)
                                & 0xf,
                        )?;
                    }

                    buf_chunk.copy_from_slice(
                        &t.chunks_exact(2)
                            .map(|chunk| chunk[0] | (chunk[1] << 4))
                            .collect::<ArrayVec<[u8; 4]>>()
                            .into_inner(),
                    );
                }
                Ok(())
            }
            SecurityLevel::TenTwoFour { .. } => {
                for (coeff_chunk, buf_chunk) in
                    self.coeffs.chunks_exact(8).zip(buf.chunks_exact_mut(5))
                {
                    for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                        let mut temp = *coeff;
                        temp += (temp >> 15) & Q_I16;
                        *t_elem = u8::try_from(
                            (((((u64::try_from(temp)?) << 5) + u64::from(Q_U32 / 2))
                                * (Q_DIV / 2))
                                >> 27)
                                & 0x1f,
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

    // Unpacks a buffer of POLYBYTES bytes into a polynomial
    // poly will NOT be normalised, but 0 <= coeffs < 4096
    // Example:
    // ```
    // unpacked_poly = Poly::unpack(buf);
    // ```
    pub fn unpack(buf: &[u8]) -> Result<Poly<Unreduced>, PackingError> {
        if buf.len() != POLYBYTES {
            return Err(CrystalsError::IncorrectBufferLength(buf.len(), POLYBYTES).into());
        }
        let coeffs_arr: [i16; N] = buf
            .chunks_exact(3)
            .flat_map(|chunk| chunk.windows(2).enumerate())
            .map(|(index, pair)| {
                if index % 2 == 0 {
                    i16::from(pair[0]) | ((i16::from(pair[1]) << 8) & 0xfff)
                } else {
                    i16::from(pair[0] >> 4) | ((i16::from(pair[1]) << 4) & 0xfff)
                }
            })
            .collect::<ArrayVec<[i16; N]>>()
            .into_inner();

        Ok(Poly {
            coeffs: coeffs_arr,
            state: Unreduced,
        })
    }

    // Converts a message buffer into a polynomial
    // msg should be of length `SYMBYTES` (32)
    // poly will not be normalised
    // Example:
    // ```
    // let read_result = Poly::read_msg(msg_buf);
    // ```
    pub(crate) fn read_msg(msg: &[u8]) -> Result<Poly<Unreduced>, PackingError> {
        if msg.len() == SYMBYTES {
            let q_plus_one_over_2 = i16::try_from((Q + 1) / 2)?;
            let coeffs_arr: [i16; N] = msg
                .iter()
                .flat_map(|&byte| (0..8).map(move |i| ((i16::from(byte) >> i) & 1).wrapping_neg()))
                .map(|mask| mask & q_plus_one_over_2)
                .collect::<ArrayVec<[i16; N]>>()
                .into_inner();

            Ok(Poly {
                coeffs: coeffs_arr,
                state: Unreduced,
            })
        } else {
            Err(CrystalsError::IncorrectBufferLength(msg.len(), SYMBYTES).into())
        }
    }

    // Decompresses buffer into a polynomial
    // is dependent on the security level
    // buf should be of length `poly_compressed_bytes`
    // output poly is normalised
    // Example:
    // ```
    // let decompress_result = Poly::decompress(buf, k);
    // ```
    pub(crate) fn decompress(buf: &[u8], sec_level: &SecurityLevel) -> Result<Self, PackingError> {
        if buf.len() != sec_level.poly_compressed_bytes() {
            return Err(CrystalsError::IncorrectBufferLength(
                buf.len(),
                sec_level.poly_compressed_bytes(),
            )
            .into());
        }

        match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                let coeffs_arr: [i16; N] = buf
                    .iter()
                    .flat_map(|&byte| {
                        (0..2).map(move |i| {
                            if i == 0 {
                                (usize::from(byte & 15) * Q + 8) >> 4
                            } else {
                                (usize::from(byte >> 4) * Q + 8) >> 4
                            }
                        })
                    })
                    .map(i16::try_from)
                    .collect::<Result<ArrayVec<[i16; N]>, TryFromIntError>>()?
                    .into_inner();

                Ok(Self {
                    coeffs: coeffs_arr,
                    state: Normalised,
                })
            }
            SecurityLevel::TenTwoFour { .. } => {
                let mut coeffs_arr = [0i16; N];
                for (coeffs_chunk, buf_chunk) in
                    coeffs_arr.chunks_exact_mut(8).zip(buf.chunks_exact(5))
                {
                    let temp: [u8; 8] = [
                        buf_chunk[0],
                        (buf_chunk[0] >> 5) | (buf_chunk[1] << 3),
                        buf_chunk[1] >> 2,
                        (buf_chunk[1] >> 7) | (buf_chunk[2] << 1),
                        (buf_chunk[2] >> 4) | (buf_chunk[3] << 4),
                        buf_chunk[3] >> 1,
                        (buf_chunk[3] >> 6) | (buf_chunk[4] << 2),
                        buf_chunk[4] >> 3,
                    ];
                    for (coeff, t_elem) in coeffs_chunk.iter_mut().zip(temp.iter()) {
                        *coeff = i16::try_from(((u32::from(*t_elem) & 31) * Q_U32 + 16) >> 5)?;
                    }
                }
                Ok(Self {
                    coeffs: coeffs_arr,
                    state: Normalised,
                })
            }
        }
    }
}
