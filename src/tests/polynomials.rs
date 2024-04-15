#![allow(warnings)]
#[cfg(test)]

mod poly_tests {
    use crate::{
        polynomials::*,
        params::*,
        tests::params::params_tests::sec_level_strategy,
    };
    use proptest::prelude::*;

    pub(in crate::tests) fn new_limited_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(-(Q as i16)..(Q as i16))
    }

    pub(in crate::tests) fn new_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(i16::MIN..i16::MAX)
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
            a in new_limited_poly_array(),
            b in new_limited_poly_array()
        ) {
            let poly_a = Poly::from_arr(&a);
            let poly_b = Poly::from_arr(&b);

            let outout = poly_a.add(&poly_b);
        }

        #[test]
        fn sub_test(
            a in new_limited_poly_array(),
            b in new_limited_poly_array()
        ) {
            let poly_a = Poly::from_arr(&a);
            let poly_b = Poly::from_arr(&b);

            let outout = poly_a.sub(&poly_b);
        }

        #[test]
        fn barrett_reduce_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);

            let output = poly.barrett_reduce();
        }

        #[test]
        fn mont_form_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);

            let output = poly.mont_form();
        }

        #[test]
        fn normalise_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);

            let output = poly.normalise();
        }

        #[test]
        fn pointwise_mul_test(
            a in new_poly_array(),
            b in new_poly_array()
        ) {
            let poly_a = Poly::from_arr(&a).normalise();
            let poly_b = Poly::from_arr(&b).normalise();

            let outout = poly_a.pointwise_mul(&poly_b);
        }

        #[test]
        fn pack_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a).normalise();

            let output = poly.pack();
        }

        #[test]
        fn write_msg_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a).normalise();
            let msg = poly.write_msg().unwrap();
        }

        #[test]
        fn compress_test(
            a in new_poly_array(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let poly = Poly::from_arr(&a).normalise();

            let result = poly.compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();
        }

        #[test]
        fn unpack_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a).normalise();
            let buf = poly.pack();

            let unpacked = Poly::unpack(&buf).unwrap();
        }

        #[test]
        fn read_msg_test(msg in prop::array::uniform32(0u8..)) {
            let poly = Poly::read_msg(&msg).unwrap();
        }

        #[test]
        fn decompress_test(
            a in new_poly_array(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let poly = Poly::from_arr(&a).normalise();

            let _ = poly.compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level);

            let decompressed_poly = Poly::decompress(&buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();
        }
    }
}
