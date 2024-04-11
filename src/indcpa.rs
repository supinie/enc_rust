use crate::{
    errors::{CrystalsError, PackingError, KeyGenerationError},
    matrix::Matrix,
    params::{SecurityLevel, K, POLYBYTES, SYMBYTES},
    polynomials::{Normalised, Montgomery, Poly},
    vectors::PolyVec,
};
use sha3::{Digest, Sha3_512};
use tinyvec::ArrayVec;

#[derive(Default, PartialEq, Debug, Eq)]
pub struct PrivateKey {
    secret: PolyVec<Normalised>,
}

#[derive(Default, PartialEq, Debug, Eq)]
pub struct PublicKey {
    rho: [u8; SYMBYTES],
    noise: PolyVec<Normalised>,
    a_t: Matrix<Montgomery>,
}

impl PrivateKey {
    fn pack(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        self.secret.pack(buf)
    }
}

fn unpack_to_private_key(buf: &[u8]) -> Result<PrivateKey, PackingError> {
    let secret = PolyVec::unpack(buf)?.normalise();
    Ok(PrivateKey { secret })
}

impl PublicKey {
    fn sec_level(&self) -> Result<SecurityLevel, CrystalsError> {
        if self.noise.sec_level() == self.a_t.sec_level() {
            Ok(self.noise.sec_level())
        } else {
            Err(CrystalsError::MismatchedSecurityLevels(
                self.noise.sec_level(),
                self.a_t.sec_level(),
            ))
        }
    }

    fn pack(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        let k: usize = self.sec_level()?.k().into();

        let break_point: usize = POLYBYTES * k;
        if buf[break_point..].len() == SYMBYTES {
            self.noise.pack(&mut buf[..break_point])?;
            buf[break_point..].copy_from_slice(&self.rho[..]);
            Ok(())
        } else {
            Err(CrystalsError::IncorrectBufferLength(buf.len(), break_point + SYMBYTES).into())
        }
    }
}

fn unpack_to_public_key(buf: &[u8]) -> Result<PublicKey, PackingError> {
    let k = K::try_from((buf.len() - SYMBYTES) / POLYBYTES)?;
    let k_value: usize = k.into();
    let break_point: usize = POLYBYTES * k_value;

    let noise = PolyVec::unpack(&buf[..break_point])?.normalise();
    let rho: [u8; SYMBYTES] = buf[break_point..].try_into()?;

    let a_t = Matrix::derive(&rho, true, k)?;

    Ok(PublicKey { rho, noise, a_t })
}

fn generate_key_pair(seed: &[u8], sec_level: SecurityLevel) -> Result<(PrivateKey, PublicKey), KeyGenerationError> {
    let mut expanded_seed = [0u8; 2 * SYMBYTES];
    let mut hash = Sha3_512::new();
    hash.update(seed);

    expanded_seed.copy_from_slice(&hash.finalize());

    let rho: [u8; SYMBYTES] = expanded_seed[..SYMBYTES].try_into()?;
    let a = Matrix::derive(&rho, false, sec_level.k())?;
    
    let sigma = &expanded_seed[32..];   // seed for noise
    
    let secret = PolyVec::derive_noise(sec_level, sigma, 0)
        .ntt()
        .normalise();

    let k_value: usize = sec_level.k().into();
    #[allow(clippy::cast_possible_truncation)]  // k_value can only be 2, 3, 4
    let error = PolyVec::derive_noise(sec_level, sigma, k_value as u8)
        .ntt();

    let noise_arr: ArrayVec<[Poly<Montgomery>; 4]> = a
        .vectors()
        .iter()
        .map(|row| row.inner_product_pointwise(&secret))
        .map(|poly| poly.mont_form())
        .collect::<ArrayVec<[Poly<Montgomery>; 4]>>();

    let noise = PolyVec::from(noise_arr)?
        .add(&error)?
        .normalise();

    let a_t = a.transpose()?;

    Ok((
        PrivateKey {
            secret,
        },
        PublicKey {
            rho,
            noise,
            a_t,
        }
    ))
}


// // pub fn encrypt<'a, PV, M>(
// pub fn encrypt<PV, M>(
//     pub_key: &PublicKey<PV, M>,
//     plaintext: &[u8],
//     seed: &[u8],
//     // output_buf: &'a mut [u8],
//     output_buf: &mut [u8],
//     // ) -> Result<&'a [u8], TryFromIntError>
// ) -> Result<(), TryFromIntError>
// where
//     PV: PolyVecOperations + GetSecLevel + Default + IntoIterator<Item = Poly> + Copy,
//     M: MatOperations + GetSecLevel + LinkSecLevel<PV> + New + IntoIterator<Item = PV> + Copy,
// {
//     let mut m = Poly::new();
//     m.read_msg(plaintext)?;

//     let mut rh = PV::new_filled();
//     rh.derive_noise(seed, 0, PV::sec_level().eta_1());
//     rh.ntt();
//     // rh.barrett_reduce();

//     let k_value: u8 = PV::sec_level().k().into();
//     let mut error_1 = PV::new_filled();
//     error_1.derive_noise(seed, k_value, PV::sec_level().eta_2());
//     let mut error_2 = Poly::new();
//     error_2.derive_noise(seed, 2 * k_value, PV::sec_level().eta_2());

//     let mut u = PV::new_filled();
//     for (mut poly, vec) in u.into_iter().zip(pub_key.a_t) {
//         poly.inner_product_pointwise(vec, rh);
//     }
//     u.inv_ntt();
//     u.add(error_1);
//     u.barrett_reduce();

//     let mut v = Poly::new();
//     v.inner_product_pointwise(pub_key.noise, rh);
//     // v.barrett_reduce();
//     v.inv_ntt();

//     v.add(&m);
//     v.add(&error_2);

//     v.barrett_reduce();

//     // u.normalise();
//     // v.normalise();

//     let poly_vec_compressed_bytes: usize = PV::sec_level().poly_vec_compressed_bytes();
//     u.compress(output_buf)?;
//     v.compress(
//         &mut output_buf[poly_vec_compressed_bytes..],
//         &PV::sec_level(),
//     )?;

//     Ok(())
// }

// // pub fn decrypt<'a, PV>(
// pub fn decrypt<PV>(
//     priv_key: &PrivateKey<PV>,
//     ciphertext: &[u8],
//     // output_buf: &'a mut [u8],
//     output_buf: &mut [u8],
//     // ) -> Result<&'a [u8], TryFromIntError>
// ) -> Result<(), TryFromIntError>
// where
//     PV: PolyVecOperations + GetSecLevel + Default + IntoIterator<Item = Poly> + Copy,
// {
//     let poly_vec_compressed_bytes: usize = PV::sec_level().poly_vec_compressed_bytes();
//     let poly_compressed_bytes: usize = PV::sec_level().poly_compressed_bytes();

//     let mut u = PV::new_filled();
//     u.decompress(&ciphertext[..poly_vec_compressed_bytes])?;
//     u.ntt();

//     let mut v = Poly::new();
//     v.decompress(
//         &ciphertext[poly_vec_compressed_bytes..poly_vec_compressed_bytes + poly_compressed_bytes],
//         &PV::sec_level(),
//     )?;

//     let mut message = Poly::new();
//     message.inner_product_pointwise(priv_key.secret, u);

//     message.barrett_reduce();
//     message.inv_ntt();
//     v.sub(&message);
//     message = v;
//     message.normalise();

//     message.write_msg(output_buf)?;

//     Ok(())
// }

// fn test() {
//     let pub_key = PublicKey {
//         rho: [0u8; 32],
//         noise: PolyVec512::from([Poly::new(); 2]),
//         a_t: [PolyVec512::from([Poly::new(); 2]); 2]
//     };

//     // let invalid_key = PublicKey {
//     //     rho: [0u8; 32],
//     //     noise: PolyVec768::from([Poly::new(); 3]),
//     //     a_t: [PolyVec512::from([Poly::new(); 2]); 2]
//     // };
// }
