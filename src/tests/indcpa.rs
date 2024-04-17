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
    use tinyvec::ArrayVec;
    use proptest::prelude::*;

    pub(in crate::tests) fn generate_random_seed() -> [u8; 32] {
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        seed
    }

    prop_compose! {
        fn new_indcpa_keypair()
            (key_seed in prop::array::uniform32(u8::MIN..u8::MAX), sec_level in sec_level_strategy())
            -> (PrivateKey, PublicKey) {
                generate_indcpa_key_pair(&key_seed, sec_level).unwrap()
            }
    }

    proptest! {
        #[test]
        fn key_gen_enc_dec(
            (priv_key, pub_key) in new_indcpa_keypair(),
            cipher_seed in prop::array::uniform32(u8::MIN..u8::MAX)
        ) {
            // let plaintext = generate_random_seed();
            let plaintext: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
            
            let ciphertext = pub_key.encrypt(&plaintext, &cipher_seed).unwrap();

            let message = priv_key.decrypt(&ciphertext).unwrap();

            // assert_eq!(message, plaintext);
        }

        #[test]
        fn key_pack_unpack(
            (priv_key, pub_key) in new_indcpa_keypair(),
        ) {
            let mut buf = [0u8; 2 * (4 * POLYBYTES) + SYMBYTES];

            let k:usize = priv_key.sec_level().k().into();
            let _ = priv_key.pack(&mut buf[..k * POLYBYTES]);
            let _ = pub_key.pack(&mut buf[k * POLYBYTES..2 * (k * POLYBYTES) + SYMBYTES]);

            let unpacked_priv = PrivateKey::unpack(&buf[..k * POLYBYTES]).unwrap();
            let unpacked_pub = PublicKey::unpack(&buf[k * POLYBYTES..2 * (k * POLYBYTES) + SYMBYTES]).unwrap();

            assert_eq!(unpacked_pub, pub_key);
            assert_eq!(unpacked_priv, priv_key);
        }

        #[test]
        #[should_panic]
        fn pub_key_pack_bad_buf_len(
            bad_bytes_len in 1..4 * POLYBYTES + SYMBYTES + 100,
            (_, pub_key) in new_indcpa_keypair(),
        ) {
            if bad_bytes_len == <K as Into<usize>>::into(pub_key.sec_level().k()) * POLYBYTES + SYMBYTES {
                panic!()
            }
            let mut bad_key_bytes = [0u8; 4 * POLYBYTES + SYMBYTES + 100];

            let pack_err = pub_key.pack(&mut bad_key_bytes[..bad_bytes_len]).unwrap();
        }

        #[test]
        #[should_panic]
        fn pub_key_unpack_bad_buf_len(
            bad_bytes_len in 1..4 * POLYBYTES + SYMBYTES + 100,
            (_, pub_key) in new_indcpa_keypair(),
        ) {
            if bad_bytes_len == <K as Into<usize>>::into(pub_key.sec_level().k()) * POLYBYTES + SYMBYTES {
                panic!()
            }
            let mut bad_key_bytes = [0u8; 4 * POLYBYTES + SYMBYTES + 100];

            let unpack_err = PublicKey::unpack(&bad_key_bytes[..bad_bytes_len]).unwrap();
        }

        #[test]
        #[should_panic]
        fn decrypt_bad_ciphertext_len(
            bad_bytes_len in 1..2048usize,
            (priv_key, _) in new_indcpa_keypair()
        ) {
            if bad_bytes_len as usize == priv_key.sec_level().indcpa_bytes() {
                panic!()
            }
            let bad_ciphertext = ArrayVec::<[u8; 2048]>::from_array_len([0u8; 2048], bad_bytes_len);
            
            let decrypt_err = priv_key.decrypt(&bad_ciphertext).unwrap();
        }
    }
}
