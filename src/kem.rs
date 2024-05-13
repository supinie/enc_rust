use crate::{
    errors::{CrystalsError, KeyGenerationError},
    indcpa::{generate_indcpa_key_pair, PrivateKey as IndcpaPrivateKey, PublicKey as IndcpaPublicKey},
    params::{SecurityLevel, SYMBYTES}
};
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
        PublicKey { pk: pk.clone(), h_pk },
        PrivateKey { sk, pk: pk.clone(), h_pk, z }
    ))
}
