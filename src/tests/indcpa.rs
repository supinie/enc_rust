#![allow(warnings)]
#[cfg(test)]
mod indcpa_tests {
    use crate::{
        indcpa::*,
        params::*,
        tests::params::params_tests::sec_level_strategy,
    };
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use proptest::prelude::*;

    pub(in crate::tests) fn generate_random_seed() -> [u8; 32] {
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        seed
    }

    proptest! {
        #[test]
        fn key_gen_enc_dec(
                key_seed in prop::array::uniform32(u8::MIN..u8::MAX),
                cipher_seed in prop::array::uniform32(u8::MIN..u8::MAX)
            ) {
            // let plaintext = generate_random_seed();
            let plaintext: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);


            let (priv_key, pub_key) = generate_key_pair(&key_seed, SecurityLevel::new(K::Three)).unwrap();
            
            let ciphertext = encrypt(&pub_key, &plaintext, &cipher_seed).unwrap();

            let message = decrypt(&priv_key, &ciphertext).unwrap();

            // assert_eq!(message, plaintext);
        }

        #[test]
        fn key_pack_unpack(
            key_seed in prop::array::uniform32(u8::MIN..u8::MAX),
            sec_level in sec_level_strategy()
        ) {
            let (priv_key, pub_key) = generate_key_pair(&key_seed, sec_level).unwrap();

            let mut buf = [0u8; 2 * (4 * POLYBYTES) + SYMBYTES];

            let k:usize = sec_level.k().into();
            let _ = priv_key.pack(&mut buf[..k * POLYBYTES]);
            let _ = pub_key.pack(&mut buf[k * POLYBYTES..2 * (k * POLYBYTES) + SYMBYTES]);

            let unpacked_priv = PrivateKey::unpack(&buf[..k * POLYBYTES]).unwrap();
            let unpacked_pub = PublicKey::unpack(&buf[k * POLYBYTES..2 * (k * POLYBYTES) + SYMBYTES]).unwrap();

            assert_eq!(unpacked_pub, pub_key);
            assert_eq!(unpacked_priv, priv_key);
        }
    }
}
