use crate::{
    field_operations::{barrett_reduce, conditional_sub_q, mont_form, montgomery_reduce},
    ntt::ZETAS,
    params::{SecurityLevel, N, Q, SYMBYTES},
};
use core::num::TryFromIntError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub(crate) coeffs: [i16; N],
}

impl Default for Poly {
    fn default() -> Self {
        Self { coeffs: [0; N] }
    }
}

impl Poly {
    // const function equivelent of default (default is needed for ArrayVec)
    // Example:
    // let poly = Poly::new();
    pub(crate) const fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    pub(crate) fn from(array: [i16; N]) -> Self {
        Self { coeffs: array }
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
        for coeff in &mut self.coeffs {
            *coeff = conditional_sub_q(barrett_reduce(*coeff));
        }
    }

    // Barrett reduces all coefficients of given polynomial
    // Example:
    // poly.barrett_reduce();
    pub(crate) fn barrett_reduce(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = barrett_reduce(*coeff);
        }
    }

    // Converts all coefficients of the given polynomial to Mongomery form
    // Example:
    // poly.mont_form();
    pub(crate) fn mont_form(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = mont_form(*coeff);
        }
    }

    // Pointwise multiplication of two polynomials,
    // assumes inputs are of montgomery form.
    // Example:
    // poly1.pointwise_mul(&poly2);
    pub(crate) fn pointwise_mul(&mut self, x: &Self) {
        for ((chunk, x_chunk), &zeta) in self.coeffs.chunks_mut(4).zip(x.coeffs.chunks(4)).zip(ZETAS.iter().skip(64)) {
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
    }

    // Packs given poly into a 384-byte (POLYBYTES size) buffer
    // must be normalised
    // Example:
    // poly.pack(buf);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub(crate) fn pack(&self, buf: &mut [u8]) {
        for i in 0..N / 2 {
            let mut t0 = self.coeffs[2 * i];
            t0 += (t0 >> 15) & Q as i16;
            let mut t1 = self.coeffs[2 * i + 1];
            t1 += (t1 >> 15) & Q as i16;

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
            self.coeffs[2 * i] = i16::from(buf[3 * i]) | ((i16::from(buf[3 * i + 1]) << 8) & 0xfff);
            self.coeffs[2 * i + 1] =
                i16::from(buf[3 * i + 1] >> 4) | ((i16::from(buf[3 * i + 2]) << 4) & 0xfff);
        }
    }

    // Converts a message buffer into a polynomial
    // msg should be of length SYMBYTES (32)
    // poly should be normalised
    // Example:
    // poly.read_msg(msg_buf);
    pub(crate) fn read_msg(&mut self, msg: &[u8]) -> Result<(), TryFromIntError> {
        for i in 0..SYMBYTES {
            for j in 0..8 {
                let mask = ((i16::from(msg[i]) >> j) & 1).wrapping_neg();
                self.coeffs[8 * i + j] = mask & i16::try_from((Q + 1) / 2)?;
            }
        }
        Ok(())
    }

    // Convert a given polynomial into a SYMBYTES (32-byte) message
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
    // buf should be of length poly_compressed_bytes
    // Example:
    // poly.decompress(buf, k);
    pub(crate) fn decompress(
        &mut self,
        buf: &[u8],
        sec_level: &SecurityLevel,
    ) -> Result<(), TryFromIntError> {
        let mut k = 0usize;

        match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                for (i, &byte) in buf.iter().take(N / 2).enumerate() {
                    self.coeffs[2 * i] = i16::try_from((usize::from(byte & 15) * Q + 8) >> 4)?;
                    self.coeffs[2 * i + 1] =
                        i16::try_from((usize::from(byte >> 4) * Q + 8) >> 4)?;
                }
                Ok(())
            }
            SecurityLevel::TenTwoFour { .. } => {
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
                        self.coeffs[8 * i + j] = i16::try_from(
                            ((u32::from(*t_elem) & 31) * u32::try_from(Q)? + 16) >> 5,
                        )?;
                    }
                }
                Ok(())
            }
        }
    }

    // Compress polynomial to a buffer
    // buf must have space for poly_compressed_bytes
    // Example:
    // self.compress(buf);
    pub(crate) fn compress(
        &self,
        buf: &mut [u8],
        sec_level: &SecurityLevel,
    ) -> Result<(), TryFromIntError> {
        let mut k = 0usize;
        let mut t = [0u8; 8];

        match sec_level {
            SecurityLevel::FiveOneTwo { .. } | SecurityLevel::SevenSixEight { .. } => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = self.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(
                            ((((u16::try_from(u)?) << 4) + u16::try_from(Q)? / 2)
                                / u16::try_from(Q)?)
                                & 15,
                        )?;
                    }

                    buf[k..k + 4].copy_from_slice(&[
                        t[0] | (t[1] << 4),
                        t[2] | (t[3] << 4),
                        t[4] | (t[5] << 4),
                        t[6] | (t[7] << 4),
                    ]);
                    k += 4;
                }
                Ok(())
            }
            SecurityLevel::TenTwoFour { .. } => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = self.coeffs[8 * i + j];
                        u += (u >> 15) & i16::try_from(Q)?;
                        t[j] = u8::try_from(
                            ((((u32::try_from(u)?) << 5) + u32::try_from(Q)? / 2)
                                / u32::try_from(Q)?)
                                & 31,
                        )?;
                    }

                    buf[k..k + 5].copy_from_slice(&[
                        t[0] | (t[1] << 5),
                        (t[1] >> 3) | (t[2] << 2) | (t[3] << 7),
                        (t[3] >> 1) | (t[4] << 4),
                        (t[4] >> 4) | (t[5] << 1) | (t[6] << 6),
                        (t[6] >> 2) | (t[7] << 3),
                    ]);
                    k += 5;
                }
                Ok(())
            }
        }
    }
}
