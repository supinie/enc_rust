#![allow(warnings)]
#[cfg(test)]
mod ntt_tests {
    use crate::{params::*, polynomials::*};
    use more_asserts::assert_lt;
    use proptest::prelude::*;

    proptest! {
        // Our poly's are over Z_q and so we only need to test for |coefficients| <= q
        #[test]
        fn ntt_test(a in prop::array::uniform(-(Q as i16)..(Q as i16))) {
            let mut poly = Poly { coeffs: a };
            poly.ntt();

            for coefficient in poly.coeffs {
                assert_lt!(coefficient.abs(), 7 * (Q as i16));
            }
        }

        #[test]
        fn inv_ntt_test(a in prop::array::uniform(-(Q as i16)..(Q as i16))) {
            let mut poly = Poly { coeffs: a };
            poly.inv_ntt();

            for coefficient in poly.coeffs {
                assert_lt!(coefficient.abs(), (Q as i16));
            }
        }

        #[test]
        fn ntt_inv_ntt_test(a in prop::array::uniform(-(Q as i16)..(Q as i16))) {
            let mut input_poly = Poly { coeffs: a };
            let mut original_input = input_poly;
            original_input.normalise();

            input_poly.ntt();
            input_poly.normalise();
            input_poly.inv_ntt();
            input_poly.normalise();

            for i in 0..N {
                let p: i32 = input_poly.coeffs[i] as i32;
                let q: i32 = original_input.coeffs[i] as i32;
                assert_eq!(
                    p,
                    (q * (1 << 16)) % (Q as i32),
                    "we are testing equality with original at index {}",
                    i
                );
            }
        }
    }
}
