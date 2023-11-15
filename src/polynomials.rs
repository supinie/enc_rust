use crate::{
    field_operations::{barrett_reduce, conditional_sub_q, mont_form, montgomery_reduce},
    ntt::ZETAS,
    params::{N, Q},
};
use core::num::TryFromIntError;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub(crate) coeffs: [i16; N],
}


#[derive(Debug, PartialEq, Eq)]
pub enum DecompressError {
    TryFromIntError,
    InvalidCompressedBytes,
}


impl From<TryFromIntError> for DecompressError {
    fn from(_err: TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}


impl Poly {
    // We can't use default, as that is only supported for arrays of length 32 or less
    // Example:
    // let poly = Poly::new();
    pub(crate) const fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    // Sets self to self + x
    // Example:
    // poly1.add(&poly2);
    pub(crate) fn add(&mut self, x: &Self) {
        for i in 0..N {
            self.coeffs[i] += x.coeffs[i];
        }
    }

    // Sets self to self - x
    // Example:
    // poly1.sub(&poly2);
    pub(crate) fn sub(&mut self, x: &Self) {
        for i in 0..N {
            self.coeffs[i] -= x.coeffs[i];
        }
    }

    // Normalise coefficients of given polynomial
    // Example:
    // poly.normalise();
    pub(crate) fn normalise(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = conditional_sub_q(barrett_reduce(*coeff));
        }
    }

    // Barrett reduces all coefficients of given polynomial
    // Example:
    // poly.reduce();
    pub(crate) fn reduce(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = barrett_reduce(*coeff);
        }
    }

    // Converts all coefficients of the given polynomial to Mongomery form
    // Example:
    // poly.mont_form();
    pub(crate) fn mont_form(&mut self) {
        for coeff in self.coeffs.iter_mut() {
            *coeff = mont_form(*coeff);
        }
    }

    // Pointwise multiplication of two polynomials,
    // assumes inputs are of montgomery form.
    // Example:
    // poly1.pointwise_mul(&poly2);
    pub(crate) fn pointwise_mul(&mut self, x: &Self) {
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
    
    
    // Packs given poly into a 384-byte buffer
    // Example:
    // poly.pack(buf);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub(crate) fn pack(&self, buf: &mut [u8]) {
        for i in 0..N / 2 {
            let t0 = self.coeffs[2 * i];
            let t1 = self.coeffs[2 * i + 1];
            
            buf[3 * i] = t0 as u8;
            buf[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            buf[3 * i + 2] = (t1 >> 4) as u8;
        }
    }

    // Unpacks a buffer of bytes into a polynomial
    // Example:
    // poly.unpack(buf);
    pub(crate) fn unpack(&mut self, buf: &[u8]) {
        for i in 0..N / 2 {
            self.coeffs[2 * i] =
                i16::from(buf[3 * i]) | ((i16::from(buf[3 * i + 1]) << 8) & 0xfff);
            self.coeffs[2 * i + 1] =
                i16::from(buf[3 * i + 1] >> 4) | (i16::from(buf[3 * i + 2]) << 4);
        }
    }

    // Converts a message buffer into a polynomial
    // Example:
    // poly.read_msg(msg_buf);
    pub(crate) fn read_msg(&mut self, msg: &[u8]) -> Result<(), TryFromIntError> {
        for i in 0..N / 8 {
            for j in 0..8 {
                let mask = ((i16::from(msg[i]) >> j) & 1).wrapping_neg();
                self.coeffs[8 * i + j] = mask & i16::try_from((Q + 1) / 2)?;
            }
        }
        Ok(())
    }


    // Convert a given polynomial into a 32-byte message
    // Example:
    // poly.write_msg(msg_buf);
    pub(crate) fn write_msg(&self, buf: &mut [u8]) -> Result<(), TryFromIntError> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let q_16 = i16::try_from(Q)?;
        for i in 0..N / 8 {
            buf[i] = 0;
            for j in 0..8 {
                let mut x = self.coeffs[8 * i + j];
                x += (x >> 15) & q_16;
                x = (((x << 1) + q_16 / 2) / q_16) & 1;
                buf[i] |= u8::try_from(x << j)?;
            }
        }
        Ok(())
    }


    // Decompresses buffer into a polynomial
    // is dependent on the security level
    // Example:
    // poly.decompress(buf, k);
    pub(crate) fn decompress(&mut self, buf: &[u8], compressed_bytes: usize) -> Result<(), DecompressError> {
        let mut k = 0usize;

        match compressed_bytes {
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
            _ => {
                Err(DecompressError::InvalidCompressedBytes)
            }
        }
    }

    // Compress selfnomial to a buffer
    // Example:
    // buf.compress(self);
    pub(crate) fn compress(& self, buf: &mut [u8], compressed_bytes: usize) -> Result<(), DecompressError> {
        let mut k = 0usize;
        let mut t = [0u8; 8];

        match compressed_bytes {
            128 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = self.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(((((u16::try_from(u)?) << 4) + u16::try_from(Q)? / 2) / u16::try_from(Q)?) & 15)?;
                    }
                    buf[k] = t[0] | (t[1] << 4);
                    buf[k + 1] = t[2] | (t[3] << 4);
                    buf[k + 2] = t[4] | (t[5] << 4);
                    buf[k + 3] = t[6] | (t[7] << 4);
                    k += 4;
                }
                Ok(())
            },
            160 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = self.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(((((u32::try_from(u)?) << 5) + u32::try_from(Q)? / 2) / u32::try_from(Q)?) & 31)?;
                    }
                    buf[k] = t[0] | (t[1] << 5);
                    buf[k + 1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
                    buf[k + 2] = (t[3] >> 1) | (t[4] << 4);
                    buf[k + 3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
                    buf[k + 4] = (t[6] >> 2) | (t[7] << 3);
                    k += 5;
                }
                Ok(())
            },
            _ => Err(DecompressError::InvalidCompressedBytes)
        }
    }
}
