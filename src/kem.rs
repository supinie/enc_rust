use crate::{
    errors::{CrystalsError, KeyGenerationError},
    indcpa::{generate_indcpa_key_pair, PrivateKey as IndcpaPrivateKey, PublicKey as IndcpaPublicKey},
    params::{SecurityLevel, SYMBYTES, K}
};
use sha3::{Digest, Sha3_256};
use rand_core::{CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

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
fn new_key_from_seed(seed: &[u8], sec_level: SecurityLevel) -> Result<(PublicKey, PrivateKey), KeyGenerationError> {
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

    Ok((
        PublicKey { pk, h_pk },
        PrivateKey { sk, pk, h_pk, z }
    ))
}

/// Generates a new keypair for a given security level.
/// Takes either a given RNG, or will generate one using `ChaCha20`
/// # Errors
/// Will return a `KeyGenerationError` if:
/// - Given invalid K value
/// - RNG fails 
/// Example:
/// ```
/// let (pk, sk) = generate_key_pair(None, 3)?;
/// ```
pub fn generate_key_pair<R: RngCore + CryptoRng>(rng: Option<&mut R>, k: usize) -> Result<(PublicKey, PrivateKey), KeyGenerationError> {
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



