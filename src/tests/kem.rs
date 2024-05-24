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
    }
}
