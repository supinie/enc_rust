#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod ntt_tests {
    use crate::{params::*, polynomials::*, tests::polynomials::poly_tests::*};
    use proptest::prelude::*;
    use more_asserts::{assert_le, assert_ge};

    proptest! {
        #[test]
        fn ntt_tests(poly in new_poly()) {
            let output_1 = poly.normalise().ntt();
            let output_2 = poly.mont_form().ntt();
            let output_3 = poly.barrett_reduce().ntt();
        }

        #[test]
        fn ntt_test_alt(poly in new_ntt_poly()) {
            let comp_poly = poly.normalise();

            poly.normalise()
                .ntt()
                .coeffs()
                .iter()
                .for_each(|&coeff| {
                    assert_le!(coeff, (7 * Q) as i16);
                    assert_ge!(coeff, -((7 * Q) as i16));
                });

            poly.normalise()
                .ntt()
                .barrett_reduce()
                .normalise()
                .inv_ntt()
                .coeffs()
                .iter()
                .for_each(|&coeff| {
                    assert_le!(coeff, Q as i16);
                    assert_ge!(coeff, -(Q as i16));
                });

            poly.normalise()
                .ntt()
                .barrett_reduce()
                .normalise()
                .inv_ntt()
                .barrett_reduce()
                .normalise()
                .coeffs()
                .iter()
                .zip(comp_poly.coeffs().iter())
                .for_each(|(&coeff, &comp_coeff)| {
                    assert_eq!(coeff as i32, ((comp_coeff as i32) * (1 << 16)) % (Q as i32));
                });
        }


        #[test]
        fn inv_ntt_test(poly in new_ntt_poly()) {
            let output = poly.inv_ntt();
        }
    }
}
