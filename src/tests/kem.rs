#![allow(warnings)]
#[cfg(test)]
mod kem_tests {
    use crate::kem::*;
    use proptest::prelude::*;

    prop_compose! {
        fn new_keypair()
            (k in 2..=4)
            -> (PublicKey, PrivateKey) {
                generate_key_pair(None, k as usize).unwrap()
            }
    }

    proptest! {
        #[test]
        fn encapsulate_decapsulate((pk, sk) in new_keypair()) {
            let (ciphertext, shared_secret) = pk.encapsulate(None, None).unwrap();

            let decap_secret = sk.decapsulate(ciphertext.as_bytes()).unwrap();

            assert_eq!(shared_secret, decap_secret);
        }

        #[test]
        fn pack_unpack((pk, sk) in new_keypair()) {
            let mut pk_bytes = [0u8; 1568];
            pk.pack(&mut pk_bytes[..pk.sec_level().public_key_bytes()]);
            let unpacked_pk = PublicKey::unpack(&pk_bytes[..pk.sec_level().public_key_bytes()]).unwrap();


            let mut sk_bytes = [0u8; 3168];
            sk.pack(&mut sk_bytes[..pk.sec_level().private_key_bytes()]);
            let unpacked_sk = PrivateKey::unpack(&sk_bytes[..sk.sec_level().private_key_bytes()]).unwrap();

            assert_eq!(pk, unpacked_pk);
            assert_eq!(sk, unpacked_sk);
        }
    }
}
