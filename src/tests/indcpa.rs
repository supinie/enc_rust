#![allow(warnings)]
#[cfg(test)]
mod indcpa_tests {
    use crate::{
        indcpa::*,
        params::*,
    };
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    pub(in crate::tests) fn generate_random_seed() -> [u8; 32] {
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        seed
    }

    #[test]
    fn key_gen_enc_dec() {
        let key_seed = generate_random_seed();
        let cipher_seed = generate_random_seed();
        let plaintext = generate_random_seed();

        let (_priv_key, pub_key) = generate_key_pair(&key_seed, SecurityLevel::new(K::Three)).unwrap();
        
        let _ = encrypt(&pub_key, &plaintext, &cipher_seed).unwrap();
    }
}
