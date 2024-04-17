#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod sample_tests {
    use crate::{
        params::*,
        polynomials::*,
        tests::{params::params_tests::sec_level_strategy, polynomials::poly_tests::*},
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn derive_noise_test(
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            nonce in (u8::MIN..u8::MAX),
            sec_level in sec_level_strategy(),
        ) {
            let poly_1 = Poly::derive_noise(&seed, nonce, sec_level.eta_1());
            let poly_2 = Poly::derive_noise(&seed, nonce, sec_level.eta_2());
        }

        #[test]
        fn derive_uniform_test(
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            x in (u8::MIN..u8::MAX),
            y in (u8::MIN..u8::MAX),
        ) {
            let poly = Poly::derive_uniform(&seed, x, y).unwrap();
        }
    }
}
