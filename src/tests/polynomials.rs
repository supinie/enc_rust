#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod poly_tests {
    use crate::{params::*, polynomials::*, tests::params::params_tests::sec_level_strategy};
    use more_asserts::assert_le;
    use proptest::prelude::*;

    pub(in crate::tests) fn new_limited_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(-(i16::MAX / 2)..(i16::MAX / 2)) // pick i16::MAX / 2, which should be plenty more
                                                              // than Q whilst ensuring no overflows (we know
                                                              // they can happen)
    }

    pub(in crate::tests) fn new_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(i16::MIN..i16::MAX)
    }

    prop_compose! {
        pub(in crate::tests) fn new_poly()
            (coeffs in new_poly_array())
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    prop_compose! {
        pub(in crate::tests) fn new_limited_poly()
            (coeffs in new_limited_poly_array())
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    prop_compose! {
        pub(in crate::tests) fn new_ntt_poly()
            (coeffs in prop::array::uniform(-(3713i16)..(3713i16)))
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    #[test]
    fn new_test() {
        let poly = Poly::new();
    }

    proptest! {
        #[test]
        fn from_arr_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);
        }

        #[test]
        fn add_test(
            a in new_limited_poly(),
            b in new_limited_poly()
        ) {
            let outout = a.add(&b);
        }

        #[test]
        fn sub_test(
            a in new_limited_poly(),
            b in new_limited_poly()
        ) {
            let outout = a.sub(&b);
        }

        #[test]
        fn barrett_reduce_test(poly in new_poly()) {
            let output = poly.barrett_reduce();
        }

        #[test]
        fn mont_form_test(poly in new_poly()) {
            let output = poly.mont_form();
        }

        #[test]
        fn normalise_test(poly in new_poly()) {
            let output = poly.normalise();
        }

        #[test]
        fn pointwise_mul_test(
            a in new_poly(),
            b in new_poly()
        ) {
            let outout = a.normalise().pointwise_mul(&b.normalise());
        }

        #[test]
        fn pack_test(poly in new_poly()) {
            let output = poly.normalise().pack();
        }

        #[test]
        fn write_msg_test(poly in new_poly()) {
            let msg = poly.normalise().write_msg().unwrap();
        }

        #[test]
        fn compress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let result = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level)
                .unwrap();
        }

        #[test]
        fn unpack_test(poly in new_poly()) {
            let buf = poly.normalise().pack();

            let unpacked = Poly::unpack(&buf).unwrap();
        }

        #[test]
        fn read_msg_test(msg in prop::array::uniform32(0u8..)) {
            let poly = Poly::read_msg(&msg).unwrap();
        }

        #[test]
        fn decompress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let _ = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level);

            let decompressed_poly = Poly::decompress(&buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();
        }

        #[test]
        fn pack_unpack_test(poly in new_poly()) {
            let buf = poly.normalise().pack();

            let unpacked = Poly::unpack(&buf).unwrap();

            assert_eq!(poly.normalise(), unpacked.normalise());
        }

        #[test]
        fn compress_decompress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160];
            let _ = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level)
                .unwrap();

            let decompressed = Poly::decompress(&buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();

            for (original_coeff, new_coeff) in poly
                .normalise()
                .coeffs()
                .iter()
                .zip(decompressed.coeffs().iter()) {
                    if (original_coeff - new_coeff).abs() < 150 {
                        assert_le!((original_coeff - new_coeff).abs(), 150, "original: {original_coeff}, new: {new_coeff}");
                    } else {
                        assert_le!(Q as i16 - (original_coeff - new_coeff).abs(), 150, "original: {original_coeff}, new: {new_coeff}");
                    }
            }
        }

        // #[test]
        // fn write_read_msg_test(poly in new_poly()) {
        //     let msg = poly.normalise().write_msg().unwrap();
        //     let new_poly = Poly::read_msg(&msg).unwrap();

        //     assert_eq!(poly, new_poly);
        // }
    }
}
