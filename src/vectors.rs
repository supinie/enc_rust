// use core::num::TryFromIntError;
use crate::{
    errors::{CrystalsError, PackingError},
    params::{SecurityLevel, K, POLYBYTES, N},
    polynomials::{
        Barrett, Montgomery, Normalised, Poly, Reduced, State, Unnormalised, Unreduced
    },
};
use tinyvec::{array_vec, ArrayVec};

#[derive(Default, PartialEq, Debug, Eq)]
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
    fn add<T: State>(&self, addend: &PolyVec<T>) -> Result<PolyVec<Unreduced>, CrystalsError> {
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
    fn barrett_reduce(&self) -> PolyVec<Barrett> {
        let mut polynomials = ArrayVec::<[Poly<Barrett>; 4]>::new();
        for poly in self.polynomials.iter() {
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
        for poly in self.polynomials.iter() {
            polynomials.push(poly.normalise());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}


impl<S: State + Reduced + Copy> PolyVec<S> {
    // apply ntt to each polynomial in the polyvec
    pub(crate) fn ntt(&self) -> PolyVec<Unreduced> {
        let mut polynomials = ArrayVec::<[Poly<Unreduced>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.ntt());
        }

        PolyVec {
            polynomials,
            sec_level: self.sec_level,
        }
    }

    // apply inv_ntt to each polynomial in the polyvec
    fn inv_ntt(&self) -> Self {
        let mut polynomials = ArrayVec::<[Poly<S>; 4]>::new();
        for poly in self.polynomials.iter() {
            polynomials.push(poly.inv_ntt());
        }

        Self {
            polynomials,
            sec_level: self.sec_level,
        }
    }
}

impl PolyVec<Normalised> {
    // Create a new, empty polyvec.
    pub(crate) fn new(k: K) -> Self {
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

    // buf should be of length k * poly_compressed_bytes
    // compresses the polyvec poly-wise into the buffer
    fn compress(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        let bytes_len = self.sec_level().poly_compressed_bytes();
        if buf.len() != self.polynomials.len() * bytes_len {
            return Err(CrystalsError::IncorrectBufferLength(
                buf.len(),
                self.polynomials.len() * bytes_len,
            )
            .into());
        }

        let _ = buf
            .chunks_mut(bytes_len)
            .zip(self.polynomials.iter())
            .map(|(buf_chunk, poly)| poly.compress(buf_chunk, &self.sec_level()));

        Ok(())
    }

    // unpack a given buffer into a polyvec poly-wise.
    // The buffer should be of length k * POLYBYTES.
    // If the length of the buffer is incorrect, the operation can still succeed provided it is a valid
    // multiple of POLYBYTES, and will result in a polyvec of incorrect security level.
    pub fn unpack(buf: &[u8]) -> Result<PolyVec<Unreduced>, PackingError> {
        let sec_level = K::try_from(buf.len() / POLYBYTES)?; // If this fails then we know the
                                                             // buffer is not of the right size and
                                                             // so no further checks are needed.

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
    // The buffer should be of length k * POLYBYTES.
    // If the length of the buffer is incorrect, the operation can still succeed provided it is a valid
    // multiple of POLYBYTES, and will result in a polyvec of incorrect security level.
    fn decompress(buf: &[u8]) -> Result<Self, PackingError> {
        let k = K::try_from(buf.len() / POLYBYTES)?;
        let sec_level = SecurityLevel::new(k);

        let polyvec_result = buf
            .chunks(sec_level.poly_compressed_bytes())
            .map(|buf_chunk| Poly::decompress(buf_chunk, &sec_level))
            .collect::<Result<ArrayVec<[Poly<Normalised>; 4]>, PackingError>>();

        match polyvec_result {
            Ok(polynomials) => Ok(Self {
                polynomials,
                sec_level: k,
            }),
            Err(err) => Err(err),
        }
    }
}

impl PolyVec<Montgomery> {
    // derive a noise polyvec using a given seed and nonce
    pub(crate) fn derive_noise(sec_level: SecurityLevel, seed: &[u8], nonce: u8) -> Self {
        let mut polynomials = ArrayVec::<[Poly<Montgomery>; 4]>::new();
        let eta = sec_level.eta_1();

        for _ in 0..sec_level.k().into() {
            polynomials.push(Poly::derive_noise(seed, nonce, eta));
        }

        Self {
            polynomials,
            sec_level: sec_level.k(),
        }
    }

    pub(crate) fn inner_product_pointwise(&self, polyvec: &Self) -> Poly<Unreduced> {
        let poly = self
            .polynomials()
            .iter()
            .zip(polyvec.polynomials())
            .map(|(&multiplicand, multiplier)| multiplicand.pointwise_mul(multiplier))
            .fold(Poly::from_arr(&[0i16; N]), |acc, x| acc.add(&x));

        poly 
    }
}


