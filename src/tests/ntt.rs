#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod ntt_tests {
    use crate::{
        polynomials::*,
        params::*,
        tests::polynomials::poly_tests::*,
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn ntt_tests(poly in new_poly()) {
            let output_1 = poly.normalise().ntt();
            let output_2 = poly.mont_form().ntt();
            let output_3 = poly.barrett_reduce().ntt();
        }

        #[test]
        fn inv_ntt_test(poly in new_ntt_poly()) {
            let output = poly.inv_ntt();
        }
    }
}
