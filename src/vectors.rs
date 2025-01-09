use crate::{
    errors::{CrystalsError, PackingError},
    params::{Eta, SecurityLevel, K, N, POLYBYTES, Q_DIV_VEC, Q_I16, Q_U32},
    polynomials::{Barrett, Montgomery, Normalised, Poly, Reduced, State, Unnormalised, Unreduced},
};
use tinyvec::ArrayVec;

#[derive(Copy, Clone, Default, PartialEq, Debug, Eq)]
pub struct PolyVec<S: State> {
    polynomials: ArrayVec<[Poly<S>; 4]>,
    sec_level: K,
}

impl<S: State> PolyVec<S> {
    // Gets the security level of the given polyvec.
    pub(crate) const fn sec_level(&self) -> SecurityLevel {
        SecurityLevel::new(self.sec_level)
    }

    // We don't expose polynomials directly to handle cases where the ArrayVec is not full for a
    // given security level. This insures we can iterate over polynomials easily.
    pub(crate) fn polynomials(&self) -> &[Poly<S>] {
        &self.polynomials.as_slice()[..self.sec_level.into()]
    }

    pub(crate) fn from(polynomials: ArrayVec<[Poly<S>; 4]>) -> Result<Self, CrystalsError> {
        K::try_from(polynomials.len()).map_or_else(
            |_| Err(CrystalsError::InternalError()),
            |sec_level| {
                Ok(Self {
                    polynomials,
                    sec_level,
                })
            },
        )
    }

    // Add two polyvecs pointwise.
    // They must be the same security level.
    pub(crate) fn add<T: State>(
        &self,
        addend: &PolyVec<T>,
    ) -> Result<PolyVec<Unreduced>, CrystalsError> {
        if self.sec_level == addend.sec_level {
            let mut polynomials = ArrayVec::<[Poly<Unreduced>; 4]>::new();
            for (augend_poly, addend_poly) in self.polynomials.iter().zip(addend.polynomials.iter())
            {
                polynomials.push(augend_poly.add(addend_poly));
            }

            Ok(PolyVec {
                polynomials,
                sec_level: self.sec_level,
            })
        } else {
            Err(CrystalsError::MismatchedSecurityLevels(
                self.sec_level(),
                addend.sec_level(),
            ))
        }
    }

    // Barrett reduce each polynomial in the polyvec
    pub(crate) fn barrett_reduce(&self) -> PolyVec<Barrett> {
        let mut polynomials = ArrayVec::<[Poly<Barrett>; 4]>::new();
        for poly in &self.polynomials {
            polynomials.push(poly.barrett_reduce());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl<S: State + Unnormalised> PolyVec<S> {
    // Normalise each polynomial in the polyvec
    pub(crate) fn normalise(&self) -> PolyVec<Normalised> {
        let mut polynomials = ArrayVec::<[Poly<Normalised>; 4]>::new();
        for poly in &self.polynomials {
            polynomials.push(poly.normalise());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl<S: State + Copy> PolyVec<S> {
    // apply inv_ntt to each polynomial in the polyvec
    pub(crate) fn inv_ntt(&self) -> Self {
        let mut polynomials = ArrayVec::<[Poly<S>; 4]>::new();
        for poly in &self.polynomials {
            polynomials.push(poly.inv_ntt());
        }

        Self {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl<S: State + Reduced + Copy> PolyVec<S> {
    // apply ntt to each polynomial in the polyvec
    pub(crate) fn ntt(&self) -> Self {
        let mut polynomials = ArrayVec::<[Poly<S>; 4]>::new();
        for poly in &self.polynomials {
            polynomials.push(poly.ntt());
        }

        Self {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl PolyVec<Normalised> {
    // buf should be of length k * POLYBYTES
    // packs the polyvec poly-wise into the buffer
    pub(crate) fn pack(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        if buf.len() != self.polynomials.len() * POLYBYTES {
            let buffer_sec_level = SecurityLevel::new(K::try_from(buf.len() / POLYBYTES)?);
            return Err(CrystalsError::MismatchedSecurityLevels(
                buffer_sec_level,
                self.sec_level(),
            )
            .into());
        }

        for (k, poly) in self.polynomials.iter().enumerate() {
            buf[k * POLYBYTES..(k + 1) * POLYBYTES].copy_from_slice(&poly.pack());
        }

        Ok(())
    }

    // buf should be of length poly_vec_compressed_bytes
    // compresses the polyvec poly-wise into the buffer
    pub(crate) fn compress(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        if buf.len() != self.sec_level().poly_vec_compressed_bytes() {
            return Err(CrystalsError::IncorrectBufferLength(
                buf.len(),
                self.sec_level().poly_vec_compressed_bytes(),
            )
            .into());
        }

        match self.sec_level() {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                let mut t = [0u16; 4];
                for (poly, buf_chunk) in self.polynomials.iter().zip(buf.chunks_exact_mut(320)) {
                    for (coeff_chunk, inner_buf_chunk) in poly
                        .coeffs()
                        .chunks_exact(4)
                        .zip(buf_chunk.chunks_exact_mut(5))
                    {
                        #[allow(
                            clippy::cast_sign_loss,
                            clippy::cast_possible_wrap,
                            clippy::cast_possible_truncation
                        )]
                        for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                            *t_elem = *coeff as u16;
                            *t_elem =
                                t_elem.wrapping_add((((*t_elem as i16) >> 15) & Q_I16) as u16);
                            *t_elem = (((((u64::from(*t_elem) << 10) + u64::from(Q_U32 / 2))
                                * Q_DIV_VEC)
                                >> 32)
                                & 0x3ff) as u16;
                        }

                        #[allow(clippy::cast_possible_truncation)]
                        inner_buf_chunk.copy_from_slice(
                            &[
                                &[t[0] as u8],
                                &t.windows(2)
                                    .enumerate()
                                    .map(|(i, t_block)| {
                                        ((t_block[0] >> (8 - 2 * i)) | (t_block[1] << (2 + 2 * i)))
                                            as u8
                                    })
                                    .collect::<ArrayVec<[u8; 3]>>()
                                    .into_inner()[..],
                                &[(t[3] >> 2) as u8],
                            ]
                            .concat(),
                        );
                    }
                }
                Ok(())
            }
            SecurityLevel::TenTwoFour { .. } => {
                let mut t = [0u16; 8];
                for (poly, buf_chunk) in self.polynomials.iter().zip(buf.chunks_exact_mut(352)) {
                    for (coeff_chunk, inner_buf_chunk) in poly
                        .coeffs()
                        .chunks_exact(8)
                        .zip(buf_chunk.chunks_exact_mut(11))
                    {
                        #[allow(
                            clippy::cast_sign_loss,
                            clippy::cast_possible_wrap,
                            clippy::cast_possible_truncation
                        )]
                        for (coeff, t_elem) in coeff_chunk.iter().zip(t.iter_mut()) {
                            *t_elem = *coeff as u16;
                            *t_elem =
                                t_elem.wrapping_add((((*t_elem as i16) >> 15) & Q_I16) as u16);
                            *t_elem = (((((u64::from(*t_elem) << 11) + u64::from(Q_U32 / 2))
                                * (Q_DIV_VEC / 2))
                                >> 31)
                                & 0x7ff) as u16;
                        }

                        #[allow(clippy::cast_possible_truncation)]
                        inner_buf_chunk.copy_from_slice(&[
                            (t[0]) as u8,
                            ((t[0] >> 8) | (t[1] << 3)) as u8,
                            ((t[1] >> 5) | (t[2] << 6)) as u8,
                            (t[2] >> 2) as u8,
                            ((t[2] >> 10) | (t[3] << 1)) as u8,
                            ((t[3] >> 7) | (t[4] << 4)) as u8,
                            ((t[4] >> 4) | (t[5] << 7)) as u8,
                            (t[5] >> 1) as u8,
                            ((t[5] >> 9) | (t[6] << 2)) as u8,
                            ((t[6] >> 6) | (t[7] << 5)) as u8,
                            (t[7] >> 3) as u8,
                        ]);
                    }
                }
                Ok(())
            }
        }
    }

    // unpack a given buffer into a polyvec poly-wise.
    // The buffer should be of length k * POLYBYTES.
    // If the length of the buffer is incorrect, the operation can still succeed provided it is a valid
    // multiple of POLYBYTES, and will result in a polyvec of incorrect security level.
    pub fn unpack(buf: &[u8]) -> Result<PolyVec<Unreduced>, PackingError> {
        let sec_level = K::try_from(buf.len() / POLYBYTES)?; // If this fails then we know the
                                                             // buffer is not of the right size and
                                                             // so no further checks are needed.;

        let polyvec_result = buf
            .chunks(POLYBYTES)
            .map(Poly::unpack)
            .collect::<Result<ArrayVec<[Poly<Unreduced>; 4]>, PackingError>>();

        match polyvec_result {
            Ok(polynomials) => Ok(PolyVec {
                polynomials,
                sec_level,
            }),
            Err(err) => Err(err),
        }
    }

    // Decompress a given buffer into a polyvec.
    // The buffer should be of length poly_vec_compressed_bytes.
    // If the length of the buffer is incorrect, the operation can still succeed provided it is a valid
    // poly_vec_compressed_bytes, and will result in a polyvec of incorrect security level.
    pub(crate) fn decompress(buf: &[u8]) -> Result<Self, PackingError> {
        let sec_level = match buf.len() {
            640 => Ok(SecurityLevel::new(K::Two)),
            960 => Ok(SecurityLevel::new(K::Three)),
            1408 => Ok(SecurityLevel::new(K::Four)),
            _ => Err(PackingError::Crystals(
                CrystalsError::IncorrectBufferLength(buf.len(), 0),
            )),
        }?;

        let polynomials = match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                let mut polys = ArrayVec::<[Poly<Normalised>; 4]>::new();

                for buf_chunk in buf.chunks_exact(320) {
                    #[allow(clippy::cast_possible_truncation)]
                    let coeffs: [i16; N] = buf_chunk
                        .windows(2)
                        .enumerate()
                        .filter(|(j, _)| j % 5 != 4)
                        .enumerate()
                        .map(|(i, (_, buf_tuple))| {
                            u16::from(buf_tuple[0] >> (2 * (i % 4)))
                                | u16::from(buf_tuple[1]) << (8 - 2 * (i % 4))
                        })
                        .map(|coeff| (((u32::from(coeff) & 0x3ff) * Q_U32 + 512) >> 10) as i16)
                        .collect::<ArrayVec<[i16; N]>>()
                        .into_inner();

                    polys.push(Poly::from_arr_normal(&coeffs));
                }

                polys
            }
            SecurityLevel::TenTwoFour { .. } => {
                let mut polys = ArrayVec::<[Poly<Normalised>; 4]>::new();

                for buf_chunk in buf.chunks_exact(352) {
                    #[allow(clippy::cast_possible_truncation)]
                    let coeffs: [i16; N] = buf_chunk
                        .chunks_exact(11)
                        .flat_map(|chunk| {
                            [
                                u16::from(chunk[0]) | u16::from(chunk[1]) << 8,
                                u16::from(chunk[1] >> 3) | u16::from(chunk[2]) << 5,
                                u16::from(chunk[2] >> 6)
                                    | u16::from(chunk[3]) << 2
                                    | u16::from(chunk[4]) << 10,
                                u16::from(chunk[4] >> 1) | u16::from(chunk[5]) << 7,
                                u16::from(chunk[5] >> 4) | u16::from(chunk[6]) << 4,
                                u16::from(chunk[6] >> 7)
                                    | u16::from(chunk[7]) << 1
                                    | u16::from(chunk[8]) << 9,
                                u16::from(chunk[8] >> 2) | u16::from(chunk[9]) << 6,
                                u16::from(chunk[9] >> 5) | u16::from(chunk[10]) << 3,
                            ]
                        })
                        .map(|coeff| ((u32::from(coeff & 0x7ff) * Q_U32 + 1024) >> 11) as i16)
                        .collect::<ArrayVec<[i16; N]>>()
                        .into_inner();

                    polys.push(Poly::from_arr_normal(&coeffs));
                }

                polys
            }
        };

        Ok(Self {
            polynomials,
            sec_level: sec_level.k(),
        })
    }
}

impl PolyVec<Montgomery> {
    // derive a noise polyvec using a given seed and nonce
    pub(crate) fn derive_noise(sec_level: SecurityLevel, seed: &[u8], nonce: u8, eta: Eta) -> Self {
        let mut polynomials = ArrayVec::<[Poly<Montgomery>; 4]>::new();

        #[allow(clippy::cast_possible_truncation)]
        for i in 0..sec_level.k().into() {
            polynomials.push(Poly::derive_noise(seed, nonce + i as u8, eta));
        }

        Self {
            polynomials,
            sec_level: sec_level.k(),
        }
    }
}

impl<S: State + Reduced + Copy> PolyVec<S> {
    pub(crate) fn inner_product_pointwise<T: State + Reduced>(
        &self,
        polyvec: &PolyVec<T>,
    ) -> Poly<Unreduced> {
        let poly = self
            .polynomials()
            .iter()
            .zip(polyvec.polynomials())
            .map(|(&multiplicand, multiplier)| multiplicand.pointwise_mul(multiplier))
            .fold(Poly::from_arr(&[0i16; N]), |acc, x| acc.add(&x));

        poly
    }
}
