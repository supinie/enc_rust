use crate::{
    errors::{CrystalsError, EncryptionDecryptionError, KeyGenerationError, PackingError},
    matrix::Matrix,
    params::{SecurityLevel, K, POLYBYTES, SYMBYTES},
    polynomials::{Montgomery, Normalised, Poly, Unreduced},
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
    const fn sec_level(&self) -> SecurityLevel {
        self.secret.sec_level()
    }

    // buf should be of length k * POLYBYTES
    pub(crate) fn pack(&self, buf: &mut [u8]) -> Result<(), PackingError> {
        self.secret.pack(buf)
    }

    // buf should be of length k * POLYBYTES
    pub(crate) fn unpack(buf: &[u8]) -> Result<Self, PackingError> {
        let secret = PolyVec::unpack(buf)?.normalise();
        Ok(Self { secret })
    }
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

    // buf should be of length k * POLYBYTES + SYMBYTES
    pub fn pack(&self, buf: &mut [u8]) -> Result<(), PackingError> {
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

    // buf should be of length k * POLYBYTES + SYMBYTES
    pub fn unpack(buf: &[u8]) -> Result<Self, PackingError> {
        let k = K::try_from((buf.len() - SYMBYTES) / POLYBYTES)?;
        let k_value: usize = k.into();
        let break_point: usize = POLYBYTES * k_value;

        let noise = PolyVec::unpack(&buf[..break_point])?.normalise();
        let rho: [u8; SYMBYTES] = buf[break_point..].try_into()?;

        let a_t = Matrix::derive(&rho, true, k)?;

        Ok(Self { rho, noise, a_t })
    }
}

pub fn generate_key_pair(
    seed: &[u8],
    sec_level: SecurityLevel,
) -> Result<(PrivateKey, PublicKey), KeyGenerationError> {
    let mut expanded_seed = [0u8; 2 * SYMBYTES];
    let mut hash = Sha3_512::new();
    hash.update(seed);

    expanded_seed.copy_from_slice(&hash.finalize());

    let rho: [u8; SYMBYTES] = expanded_seed[..SYMBYTES].try_into()?;
    let a = Matrix::derive(&rho, false, sec_level.k())?;

    let sigma = &expanded_seed[32..]; // seed for noise

    let secret = PolyVec::derive_noise(sec_level, sigma, 0, sec_level.eta_1())
        .ntt()
        .normalise();

    let k_value: usize = sec_level.k().into();
    #[allow(clippy::cast_possible_truncation)] // k_value can only be 2, 3, 4
    let error = PolyVec::derive_noise(sec_level, sigma, k_value as u8, sec_level.eta_1())
        .ntt();

    let noise_arr: ArrayVec<[Poly<Montgomery>; 4]> = a
        .vectors()
        .iter()
        .map(|row| row.inner_product_pointwise(&secret))
        .map(|poly| poly.mont_form())
        .collect::<ArrayVec<[Poly<Montgomery>; 4]>>();

    let noise = PolyVec::from(noise_arr)?.add(&error)?.normalise();


    let a_t = a.transpose()?;

    Ok((PrivateKey { secret }, PublicKey { rho, noise, a_t }))
}

pub fn encrypt(
    pub_key: &PublicKey,
    message: &[u8], // length SYMBYTES
    seed: &[u8],    // length SYMBYTES
) -> Result<ArrayVec<[u8; 2048]>, EncryptionDecryptionError> {
    // must be able to contain max
    // indcpa bytes, but trait bounds
    // only satisfied for powers of 2
    // when > 32.
    let sec_level = pub_key.sec_level()?;
    let k_value: usize = sec_level.k().into();
    let msg_poly = Poly::read_msg(message)?;

    let rh = PolyVec::derive_noise(sec_level, seed, 0, sec_level.eta_1())
        .ntt()
        .barrett_reduce();

    #[allow(clippy::cast_possible_truncation)] // k_value will never be truncated
    let error_1 = PolyVec::derive_noise(sec_level, seed, k_value as u8, sec_level.eta_2());
    #[allow(clippy::cast_possible_truncation)] // k_value will never be truncated
    let error_2 = Poly::derive_noise(seed, (k_value as u8) * 2, sec_level.eta_2());

    //  u = A_t r + e_1
    let u = PolyVec::from(
        pub_key
            .a_t
            .vectors()
            .iter()
            .map(|row| row.inner_product_pointwise(&rh))
            .collect::<ArrayVec<[Poly<Unreduced>; 4]>>(),
        )?
        .barrett_reduce()
        .inv_ntt()
        .add(&error_1)?
        .normalise();

    //  v = <t, r> + e_2 + message
    let v = pub_key
        .noise
        .inner_product_pointwise(&rh)
        .barrett_reduce()
        .inv_ntt()
        .add(&msg_poly)
        .add(&error_2)
        .normalise();

    let mut ciphertext_bytes = ArrayVec::from_array_len([0u8; 2048], sec_level.indcpa_bytes());

    let (u_bytes, v_bytes) = ciphertext_bytes.split_at_mut(sec_level.poly_vec_compressed_bytes());
    u.compress(u_bytes)?;
    v.compress(v_bytes, &sec_level)?;

    Ok(ciphertext_bytes)
}

pub fn decrypt(
    priv_key: &PrivateKey,
    ciphertext: &ArrayVec<[u8; 2048]>,
) -> Result<[u8; SYMBYTES], EncryptionDecryptionError> {
    let sec_level = priv_key.sec_level();
    if ciphertext.len() == sec_level.indcpa_bytes() {
        let (u_bytes, v_bytes) = ciphertext.split_at(sec_level.poly_vec_compressed_bytes());
        let u = PolyVec::decompress(u_bytes)?.ntt();
        let v = Poly::decompress(v_bytes, &sec_level)?;

        //  m = v - <s, u>
        let m = v
            .sub(
                &priv_key
                    .secret
                    .inner_product_pointwise(&u)
                    .barrett_reduce()
                    .inv_ntt()
            )
            .normalise();

        Ok(m.write_msg()?)
    } else {
        Err(CrystalsError::IncorrectBufferLength(ciphertext.len(), sec_level.indcpa_bytes()).into())
    }
}
