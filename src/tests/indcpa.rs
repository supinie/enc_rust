#[cfg(test)]
mod indcpa_tests {
    use crate::{indcpa::*, vectors::PolyVec512, matrix::{Mat512, MatOperations}, tests::sample::sample_tests::generate_random_seed, polynomials::Poly, params::{N, SYMBYTES}};

    #[test]
    fn priv_pack_unpack() {
        let priv_key = PrivateKey::<PolyVec512> {
            secret: PolyVec512::from([Poly { coeffs: [20; N] }; 2]),
        };
        let mut buf = [0u8; 768];
        priv_key.pack(&mut buf);
        let mut new_priv_key = PrivateKey::<PolyVec512> {
            secret: PolyVec512::from([Poly { coeffs: [0; N] }; 2]),
        };
        new_priv_key.unpack(&buf);
        assert_eq!(priv_key, new_priv_key);
    }

    #[test]
    fn pub_pack_unpack() {
        let seed = generate_random_seed();
        let pub_key = PublicKey::<PolyVec512, Mat512> {
            rho: seed,
            noise: PolyVec512::from([Poly { coeffs: [20; N]}; 2]),
            a_t: Mat512::derive(&seed, true),
        };
        let mut buf = [0u8; 2 * 384 + 32]; // k * poly_compressed_bytes + rho length
        pub_key.pack(&mut buf);
        let mut new_pub_key = PublicKey::<PolyVec512, Mat512> {
            rho: [0u8; 32],
            noise: PolyVec512::from([Poly { coeffs: [0; N]}; 2]),
            a_t: Mat512::derive(&generate_random_seed(), true),
        };
        new_pub_key.unpack(&buf);
        assert_eq!(pub_key, new_pub_key);
    }


    #[test]
    fn key_gen_enc_dec() {
        let key_seed = generate_random_seed();
        let cipher_seed = generate_random_seed();
        let plaintext = generate_random_seed();

        let (priv_key, pub_key) = generate_key_pair::<PolyVec512, Mat512>(&key_seed);

        let mut ciphertext = [0u8; 768]; //indcpa bytes

        let _ = encrypt(&pub_key, &plaintext, &cipher_seed, &mut ciphertext);

        let mut message = [0u8; SYMBYTES]; // SYMBYTES
        let _ = decrypt(&priv_key, &ciphertext, &mut message);

        // assert_eq!(ciphertext, [0u8; 768]);
        assert_eq!(message, plaintext);
    }
}
