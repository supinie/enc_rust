// #[cfg(test)]
// mod indcpa_tests {
//     use crate::{indcpa::*, vectors::PolyVec512, matrix::Mat512, tests::sample::sample_tests::generate_random_seed};

//     #[test]
//     fn key_gen_enc_dec() {
//         let seed = generate_random_seed();
//         let plaintext = generate_random_seed();

//         let (priv_key, pub_key) = generate_key_pair::<PolyVec512, Mat512>(&seed);

//         let mut ciphertext = [0u8; 768];

//         let _ = encrypt(&pub_key, &plaintext, &seed, &mut ciphertext);

//         let mut message = [0u8; 768];
//         let _ = decrypt(&priv_key, &ciphertext, &mut message);

//         assert_eq!(message, ciphertext);
//     }
// }
