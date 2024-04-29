#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod ntt_tests {
    use crate::{params::*, polynomials::*, tests::polynomials::poly_tests::*};
    use proptest::prelude::*;
    use more_asserts::{assert_le, assert_ge, assert_lt};

    #[rustfmt::skip]
    const INV_NTT_REDUCTIONS: [&[usize]; 7] = [
        &[],
        &[],
        &[16, 17, 48, 49, 80, 81, 112, 113, 144, 145, 176, 177, 208, 209, 240, 241],
        &[0, 1, 32, 33, 34, 35, 64, 65, 96, 97, 98, 99, 128, 129, 160, 161, 162, 163, 192, 193, 224, 225, 226, 227],
        &[2, 3, 66, 67, 68, 69, 70, 71, 130, 131, 194, 195, 196, 197, 198, 199],
        &[4, 5, 6, 7, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143],
        &[]
    ];


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

        #[test]
        fn inv_ntt_test_alt(poly in new_ntt_poly()) {
            let mut xs = [1i16; 256];

            let mut r = -1;

            for (layer, reductions) in (1..8).zip(INV_NTT_REDUCTIONS) {
                let w = 1 << layer;
                let mut i = 0;

                if i + w < 256 {
                    xs[i] = xs[i] + xs[i + w];
                    assert_lt!(xs[i], 9);

                    xs[i + w] = 1;
                    i += 1;

                    if i % w == 0 {
                        i += w;
                    }
                }
                
                for &i in reductions {
                    xs[i] = 1;
                }
            }
        }
    }
}
