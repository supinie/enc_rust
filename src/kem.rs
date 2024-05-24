use crate::{
    errors::{CrystalsError, EncryptionDecryptionError, KeyGenerationError},
    indcpa::{
        generate_indcpa_key_pair, PrivateKey as IndcpaPrivateKey, PublicKey as IndcpaPublicKey,
    },
    params::{SecurityLevel, K, SHAREDSECRETBYTES, SYMBYTES, MAX_CIPHERTEXT},
};
use rand_chacha::ChaCha20Rng;
use rand_core::{CryptoRng, RngCore, SeedableRng};
use sha3::{Digest, Sha3_256, Sha3_512, Shake256, digest::{ExtendableOutput, Update, XofReader}};
use subtle::{ConstantTimeEq, ConditionallySelectable};
use tinyvec::ArrayVec;

#[derive(Debug)]
pub struct PrivateKey {
    sk: IndcpaPrivateKey,
    pk: IndcpaPublicKey,
    h_pk: [u8; SYMBYTES],
    z: [u8; SYMBYTES],
}

#[derive(Debug)]
pub struct PublicKey {
    pk: IndcpaPublicKey,
    h_pk: [u8; SYMBYTES],
}

pub struct Ciphertext {
    bytes: [u8; MAX_CIPHERTEXT], // max ciphertext_bytes()
    len: usize,
}

impl Ciphertext {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

fn sha3_256_from(input: &[u8]) -> [u8; SYMBYTES] {
    let mut hash = Sha3_256::new();
    Digest::update(&mut hash, input);

    let output: [u8; SYMBYTES] = hash.finalize().into();
    output
}

fn sha3_512_from(input: &[u8]) -> ([u8; SHAREDSECRETBYTES], [u8; SYMBYTES]) {
    let mut hash = Sha3_512::new();
    Digest::update(&mut hash, input);
    let output = hash.finalize();

    let mut o1 = [0u8; SHAREDSECRETBYTES];
    let mut o2 = [0u8; SYMBYTES];

    o1.copy_from_slice(&output[..SHAREDSECRETBYTES]);
    o2.copy_from_slice(&output[SHAREDSECRETBYTES..]);
    (o1, o2)
}

fn shake256_from(input: &[u8]) -> [u8; SHAREDSECRETBYTES] {
    let mut hash = Shake256::default();
    hash.update(input);
    let mut output = [0u8; SHAREDSECRETBYTES];
    hash.finalize_xof().read(&mut output);
    output
}

// derived new keypair deterministically from a given 64 (2 * 32) byte seed.
fn new_key_from_seed(
    seed: &[u8],
    sec_level: SecurityLevel,
) -> Result<(PublicKey, PrivateKey), KeyGenerationError> {
    if seed.len() != 2 * SYMBYTES {
        return Err(CrystalsError::InvalidSeedLength(seed.len(), 2 * SYMBYTES).into());
    }

    let (sk, pk) = generate_indcpa_key_pair(&seed[..SYMBYTES], sec_level)?;

    let z: [u8; SYMBYTES] = seed[SYMBYTES..].try_into()?;

    let mut packed_pk = [0u8; MAX_CIPHERTEXT]; // max packed public key size
    pk.pack(&mut packed_pk[..sec_level.indcpa_public_key_bytes()])?;

    let h_pk: [u8; SYMBYTES] = sha3_256_from(&packed_pk[..sec_level.indcpa_public_key_bytes()]);

    Ok((PublicKey { pk, h_pk }, PrivateKey { sk, pk, h_pk, z }))
}

pub trait AcceptableRng: RngCore + CryptoRng {}

/// Generates a new keypair for a given security level.
/// Takes either a given RNG, or will generate one using `ChaCha20`
/// # Errors
/// Will return a `KeyGenerationError` if:
/// - Given invalid K value
/// - RNG fails
/// Example:
/// ```
/// use enc_rust::kem::generate_key_pair;
///
/// let (pk, sk) = generate_key_pair(None, 3)?;
///
/// # Ok::<(), enc_rust::errors::KeyGenerationError>(())
/// ```
pub fn generate_key_pair(
    rng: Option<&mut dyn AcceptableRng>,
    k: usize,
) -> Result<(PublicKey, PrivateKey), KeyGenerationError> {
    let k_result = K::try_from(k);

    if let Ok(k_value) = k_result {
        let mut seed = [0u8; 2 * SYMBYTES];

        if let Some(rng) = rng {
            rng.try_fill_bytes(&mut seed)?;
        } else {
            let mut chacha = ChaCha20Rng::from_entropy();
            chacha.try_fill_bytes(&mut seed)?;
        };

        let sec_level = SecurityLevel::new(k_value);

        return new_key_from_seed(&seed, sec_level);
    }

    Err(CrystalsError::InvalidK(k).into())
}

impl PrivateKey {
    const fn sec_level(&self) -> SecurityLevel {
        self.sk.sec_level()
    }

    #[must_use]
    pub const fn get_public_key(&self) -> PublicKey {    
        PublicKey {
            pk: self.pk,
            h_pk: self.h_pk,
        }
    }

    pub fn decapsulate(&self, ciphertext: &Ciphertext) -> Result<[u8; SHAREDSECRETBYTES], EncryptionDecryptionError> {
        let sec_level = self.sec_level();

        if ciphertext.len != sec_level.ciphertext_bytes() {
            return Err(CrystalsError::InvalidCiphertextLength(ciphertext.len, sec_level.ciphertext_bytes(), sec_level.k()).into());
        }
        
        let m = self.sk.decrypt(ciphertext.as_bytes())?;

        let (k, r) = sha3_512_from(&[m, self.h_pk].concat());

        let k_bar = shake256_from(&[&self.z, ciphertext.as_bytes()].concat());

        let mut ct = [0u8; MAX_CIPHERTEXT]; // max indcpa_bytes()
        self.pk.encrypt(&m, &r, &mut ct[..sec_level.indcpa_bytes()])?;

        let equal = ct.ct_eq(ciphertext.as_bytes());

        Ok(k.iter()
           .zip(k_bar.iter())
           .map(|(x, y)| u8::conditional_select(x, y, equal))
           .collect::<ArrayVec<[u8; SHAREDSECRETBYTES]>>()
           .into_inner()
        )

    }
}

impl PublicKey {
    pub fn encapsulate(
        &self,
        seed: Option<&[u8]>,
        rng: Option<&mut dyn AcceptableRng>,
    ) -> Result<(Ciphertext, [u8; SHAREDSECRETBYTES]), EncryptionDecryptionError> {
        let sec_level = self.pk.sec_level();

        let mut m = [0u8; SYMBYTES];
        if let Some(seed) = seed {
            if seed.len() != SYMBYTES {
                return Err(CrystalsError::InvalidSeedLength(seed.len(), SYMBYTES).into());
            }
            m.copy_from_slice(seed);
        } else if let Some(rng) = rng {
            rng.try_fill_bytes(&mut m)?;
        } else {
            let mut chacha = ChaCha20Rng::from_entropy();
            chacha.try_fill_bytes(&mut m)?;
        }

        let (k, r) = sha3_512_from(&[m, self.h_pk].concat());
        let mut bytes = [0u8; MAX_CIPHERTEXT]; // max ciphertext_bytes
        self.pk.encrypt(&m, &r, &mut bytes[..sec_level.ciphertext_bytes()])?;
        
        Ok((
            Ciphertext {
                bytes,
                len: sec_level.ciphertext_bytes(),
            },
            k
        ))
    }
}
