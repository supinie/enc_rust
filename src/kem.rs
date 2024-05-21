use crate::{
    errors::{CrystalsError, EncryptionDecryptionError, KeyGenerationError},
    indcpa::{
        generate_indcpa_key_pair, PrivateKey as IndcpaPrivateKey, PublicKey as IndcpaPublicKey,
    },
    params::{SecurityLevel, K, SHAREDSECRETBYTES, SYMBYTES},
};
use rand_chacha::ChaCha20Rng;
use rand_core::{CryptoRng, RngCore, SeedableRng};
use sha3::{Digest, Sha3_256};

pub struct PrivateKey {
    sk: IndcpaPrivateKey,
    pk: IndcpaPublicKey,
    h_pk: [u8; SYMBYTES],
    z: [u8; SYMBYTES],
}

pub struct PublicKey {
    pk: IndcpaPublicKey,
    h_pk: [u8; SYMBYTES],
}

// derived new keypair deterministically from a given 64 (2 * 32) byte seed.
fn new_key_from_seed(
    seed: &[u8],
    sec_level: SecurityLevel,
) -> Result<(PublicKey, PrivateKey), KeyGenerationError> {
    if seed.len() != 2 * SYMBYTES {
        return Err(CrystalsError::InvalidSeedLength(seed.len(), 2 * SYMBYTES).into());
    }

    let (sk, pk) = generate_indcpa_key_pair(&seed[..32], sec_level)?;

    let z: [u8; SYMBYTES] = seed[32..].try_into()?;

    let mut packed_pk = [0u8; 1440]; // max packed public key size
    pk.pack(&mut packed_pk[..sec_level.indcpa_public_key_bytes()])?;

    let mut hash = Sha3_256::new();
    hash.update(&packed_pk[..sec_level.indcpa_public_key_bytes()]);

    let h_pk: [u8; SYMBYTES] = hash.finalize().into();

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
    fn sec_level(&self) -> SecurityLevel {
        self.sk.sec_level()
    }

    pub fn get_public_key(&self) -> PublicKey {    
        PublicKey {
            pk: self.pk,
            h_pk: self.h_pk,
        }
    }

    // pub fn decapsulate(&self, ciphertext: &[u8]) -> Result<[u8; SHAREDSECRETBYTES], EncryptionDecryptionError> {
    //     let sec_level = self.sec_level();

    //     if ciphertext.len() != sec_level.ciphertext_bytes() {
    //         Err(CrystalsError::InvalidCiphertextLength(ciphertext.len(), sec_levl.ciphertext_bytes(), sec_level.k()).into())
    //     }
        
    //     let m = self.sk.decrypt(&

}

#[cfg(target_os = "none")]
impl PublicKey {
    pub fn encapsulate<R: RngCore + CryptoRng>(
        &self,
        seed: Option<&[u8]>,
        shared_secret: Option<[u8; SHAREDSECRETBYTES]>,
        rng: Option<&mut R>,
    ) -> Result<(CIPHERTEXT, SHAREDSECRET), EncryptionDecryptionError> {
        if let Some(seed) = seed {
            if seed.len() != SYMBYTES {
                Err(CrystalsError::InvalidSeedLength(seed.len(), SYMBYTES).into())
            }
            Ok(())
        }
    }
}