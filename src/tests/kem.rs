#![allow(warnings)]
#[cfg(test)]
mod kem_tests {
    use crate::kem::*;
    use proptest::prelude::*;

    prop_compse! {
        fn new_keypair()
            (k in 2..=4)
            -> (PublicKey, PrivateKey) {
                generate_key_pair(None, k).unwrap()
            }
    }

    proptest! {
        #[test]
        fn encapsulate_decapsulate((pk, sk) in new_keypair()) {
            let (ciphertext, shared_secret) = pk.encapsulate().unwrap();
            
            let decap_secret = sk.decapsulate(ciphertext).unwrap();

            assert_eq!(shared_secret, decap_secret);
        }
    }
}
