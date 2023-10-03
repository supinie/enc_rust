use crate::{
    buffer::Buffer,
    field_ops::*,
    ntt::ZETAS,
    params::{N, Q},
};
use core::num::TryFromIntError;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Poly {
    pub coeffs: [i16; N],
}


#[derive(Debug)]
pub enum DecompressError {
    TryFromIntError,
    InvalidCompressedBytes,
}


impl From<TryFromIntError> for DecompressError {
    fn from(_err: TryFromIntError) -> Self {
        DecompressError::TryFromIntError
    }
}


impl Poly {
    // We can't use default, as that is only supported for arrays of length 32 or less
    // Example:
    // let poly = Poly::new();
    pub fn new() -> Self {
        Poly { coeffs: [0; N] }
    }

    // Sets self to self + x
    // Example:
    // poly1.add(&poly2);
    pub fn add(&mut self, x: &Poly) {
        for i in 0..N {
            self.coeffs[i] += x.coeffs[i];
        }
    }

    // Sets self to self - x
    // Example:
    // poly1.sub(&poly2);
    pub fn sub(&mut self, x: &Poly) {
        for i in 0..N {
            self.coeffs[i] -= x.coeffs[i];
        }
    }

    // Normalise coefficients of given polynomial
    // Example:
    // poly.normalise();
    pub fn normalise(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = cond_sub_q(barrett_reduce(*coeff));
        }
    }

    // Barrett reduces all coefficients of given polynomial
    // Example:
    // poly.reduce();
    pub fn reduce(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = barrett_reduce(*coeff);
        }
    }

    // Converts all coefficients of the given polynomial to Mongomery form
    // Example:
    // poly.mont_form();
    pub fn mont_form(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = mont_form(*coeff);
        }
    }

    // Pointwise multiplication of two polynomials,
    // assumes inputs are of montgomery form.
    // Example:
    // poly1.pointwise_mul(&poly2);
    pub fn pointwise_mul(&mut self, x: &Poly) {
        let mut j: usize = 64;

        for i in (0..N).step_by(4) {
            let zeta = i32::from(ZETAS[j]);
            j += 1;

            let mut p0 = montgomery_reduce(i32::from(self.coeffs[i + 1]) * i32::from(x.coeffs[i + 1]));
            p0 = montgomery_reduce(i32::from(p0) * zeta);
            p0 += montgomery_reduce(i32::from(self.coeffs[i]) * i32::from(x.coeffs[i]));

            let mut p1 = montgomery_reduce(i32::from(self.coeffs[i]) * i32::from(x.coeffs[i + 1]));
            p1 += montgomery_reduce(i32::from(self.coeffs[i + 1]) * i32::from(x.coeffs[i]));

            let mut p2 = montgomery_reduce(i32::from(self.coeffs[i + 3]) * i32::from(x.coeffs[i + 3]));
            p2 = -montgomery_reduce(i32::from(p2) * zeta);
            p2 += montgomery_reduce(i32::from(self.coeffs[i + 2]) * i32::from(x.coeffs[i + 2]));

            let mut p3 = montgomery_reduce(i32::from(self.coeffs[i + 2]) * i32::from(x.coeffs[i + 3]));
            p3 += montgomery_reduce(i32::from(self.coeffs[i + 3]) * i32::from(x.coeffs[i + 2]));

            self.coeffs[i] = p0;
            self.coeffs[i + 1] = p1;
            self.coeffs[i + 2] = p2;
            self.coeffs[i + 3] = p3;
        }
    }

    // Unpacks a buffer of bytes into a polynomial
    // Example:
    // poly.unpack(buf);
    pub fn unpack(&mut self, buf: &[u8]) {
        for i in 0..N / 2 {
            self.coeffs[2 * i] =
                i16::from(buf[3 * i]) | ((i16::from(buf[3 * i + 1]) << 8) & 0xfff);
            self.coeffs[2 * i + 1] =
                i16::from(buf[3 * i + 1] >> 4) | (i16::from(buf[3 * i + 2]) << 4);
        }
    }

    // Converts a message buffer into a polynomial
    // Example:
    // poly.load_msg(msg_buf);
    pub fn load_msg(&mut self, msg: &[u8]) -> Result<(), TryFromIntError> {
        for i in 0..N / 8 {
            for j in 0..8 {
                let mask = ((i16::from(msg[i]) >> j) & 1).wrapping_neg();
                self.coeffs[8 * i + j] = mask & i16::try_from((Q + 1) / 2)?;
            }
        }
        Ok(())
    }


    // Decompresses buffer into a polynomial
    // is dependent on the security level
    // Example:
    // poly.decompress(buf, k);
    pub fn decompress(&mut self, buf: &[u8], compressed_bytes: Option<usize>) -> Result<(), DecompressError> {
        let mut k = 0usize;

        match compressed_bytes {
            Some(size) => match size {
                128 => {
                    for i in 0..N / 2 {
                        self.coeffs[2 * i] = i16::try_from((usize::from(buf[k] & 15) * Q + 8) >> 4)?;
                        self.coeffs[2 * i + 1] = i16::try_from((usize::from(buf[k] >> 4) * Q + 8) >> 4)?;
                        k += 1;
                    };
                    Ok(())
                }
                160 => {
                    let mut t = [0u8; 8];
                    for i in 0..N / 8 {
                        t[0] = buf[k];
                        t[1] = (buf[k] >> 5) | (buf[k + 1] << 3);
                        t[2] = buf[k + 1] >> 2;
                        t[3] = (buf[k + 1] >> 7) | (buf[k + 2] << 1);
                        t[4] = (buf[k + 2] >> 4) | (buf[k + 3] << 4);
                        t[5] = buf[k + 3] >> 1;
                        t[6] = (buf[k + 3] >> 6) | (buf[k + 4] << 2);
                        t[7] = buf[k + 4] >> 3;
                        k += 5;

                        for (j, t_elem) in t.iter().enumerate() {
                            self.coeffs[8 * i + j] =
                                i16::try_from(((u32::from(*t_elem) & 31) * u32::try_from(Q)? + 16) >> 5)?;
                        }
                    };
                    Ok(())
                }
                _ => Err(DecompressError::InvalidCompressedBytes)
            }
            None => Err(DecompressError::InvalidCompressedBytes)
        }
    }
}
