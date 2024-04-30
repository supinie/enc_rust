#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod poly_tests {
    use crate::{params::*, polynomials::*, tests::params::params_tests::sec_level_strategy};
    use more_asserts::assert_le;
    use proptest::prelude::*;

    const compress_decompress_buf: [u8; 128] = [
        0, 0, 0, 0, 0, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 34, 34, 34, 34, 34, 34, 34, 34,
        34, 34, 50, 51, 51, 51, 51, 51, 51, 51, 51, 51, 67, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68,
        85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 101, 102, 102, 102, 102, 102, 102, 102, 102, 102,
        102, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 135, 136, 136, 136, 136, 136, 136,
        136, 136, 136, 152, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 170, 170, 170, 170,
        170, 170, 170, 170, 170, 170, 186, 187, 187, 187, 187, 187, 187, 187, 187, 187, 187, 204,
        204, 204, 204, 204, 204, 204, 204,
    ];

    const pack_unpack_buf: [u8; 384] = [
        0, 160, 0, 20, 224, 1, 40, 32, 3, 60, 96, 4, 80, 160, 5, 100, 224, 6, 120, 32, 8, 140, 96,
        9, 160, 160, 10, 180, 224, 11, 200, 32, 13, 220, 96, 14, 240, 160, 15, 4, 225, 16, 24, 33,
        18, 44, 97, 19, 64, 161, 20, 84, 225, 21, 104, 33, 23, 124, 97, 24, 144, 161, 25, 164, 225,
        26, 184, 33, 28, 204, 97, 29, 224, 161, 30, 244, 225, 31, 8, 34, 33, 28, 98, 34, 48, 162,
        35, 68, 226, 36, 88, 34, 38, 108, 98, 39, 128, 162, 40, 148, 226, 41, 168, 34, 43, 188, 98,
        44, 208, 162, 45, 228, 226, 46, 248, 34, 48, 12, 99, 49, 32, 163, 50, 52, 227, 51, 72, 35,
        53, 92, 99, 54, 112, 163, 55, 132, 227, 56, 152, 35, 58, 172, 99, 59, 192, 163, 60, 212,
        227, 61, 232, 35, 63, 252, 99, 64, 16, 164, 65, 36, 228, 66, 56, 36, 68, 76, 100, 69, 96,
        164, 70, 116, 228, 71, 136, 36, 73, 156, 100, 74, 176, 164, 75, 196, 228, 76, 216, 36, 78,
        236, 100, 79, 0, 165, 80, 20, 229, 81, 40, 37, 83, 60, 101, 84, 80, 165, 85, 100, 229, 86,
        120, 37, 88, 140, 101, 89, 160, 165, 90, 180, 229, 91, 200, 37, 93, 220, 101, 94, 240, 165,
        95, 4, 230, 96, 24, 38, 98, 44, 102, 99, 64, 166, 100, 84, 230, 101, 104, 38, 103, 124,
        102, 104, 144, 166, 105, 164, 230, 106, 184, 38, 108, 204, 102, 109, 224, 166, 110, 244,
        230, 111, 8, 39, 113, 28, 103, 114, 48, 167, 115, 68, 231, 116, 88, 39, 118, 108, 103, 119,
        128, 167, 120, 148, 231, 121, 168, 39, 123, 188, 103, 124, 208, 167, 125, 228, 231, 126,
        248, 39, 128, 12, 104, 129, 32, 168, 130, 52, 232, 131, 72, 40, 133, 92, 104, 134, 112,
        168, 135, 132, 232, 136, 152, 40, 138, 172, 104, 139, 192, 168, 140, 212, 232, 141, 232,
        40, 143, 252, 104, 144, 16, 169, 145, 36, 233, 146, 56, 41, 148, 76, 105, 149, 96, 169,
        150, 116, 233, 151, 136, 41, 153, 156, 105, 154, 176, 169, 155, 196, 233, 156, 216, 41,
        158, 236, 105, 159,
    ];

    const msg_buf: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 3,
    ];

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
    fn compare_compress_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);

        let poly = Poly::from_arr(&coeffs);
        let mut buf = [0u8; 128];
        let _ = poly
            .normalise()
            .compress(&mut buf, &SecurityLevel::new(K::Three))
            .unwrap();

        assert_eq!(buf, compress_decompress_buf);
    }

    #[test] fn compare_decompress_test() {
        let poly = Poly::decompress(&compress_decompress_buf, &SecurityLevel::new(K::Three)).unwrap();

        let coeffs: [i16; N] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497];
        let comp_poly = Poly::from_arr(&coeffs).normalise();

        assert_eq!(poly, comp_poly);
    }
    

    #[test]
    fn compare_pack_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);

        let poly = Poly::from_arr(&coeffs);
        let buf = poly.normalise().pack();

        assert_eq!(buf, pack_unpack_buf);
    }

    #[test]
    fn compare_unpack_test() {
        let poly = Poly::unpack(&pack_unpack_buf).unwrap();

        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);
        let comp_poly = Poly::from_arr(&coeffs);

        assert_eq!(poly, comp_poly);
    }

    #[test]
    fn compare_msg_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);
        let poly = Poly::from_arr(&coeffs);
        let buf = poly.normalise().write_msg().unwrap();

        assert_eq!(buf, msg_buf);
    }

    #[test]
    fn compare_read_msg_test() {
        let poly = Poly::read_msg(&msg_buf).unwrap().normalise();

        let comp_poly = Poly::from_arr(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 0, 0, 0, 0, 0, 0]).normalise();
        assert_eq!(poly, comp_poly);
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

        // #[test]
        // fn write_msg_test_alt(poly in new_poly()) {
        //     let msg = poly.normalise().write_msg().unwrap();
        //     if poly.coeffs()[0] >= 833 && poly.coeffs()[0] < 2497 {
        //         assert_eq!(msg[0], 1u8);
        //     } else {
        //         assert_eq!(msg[0], 0u8);
        //     }
        // }

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

        #[test]
        fn write_read_msg_test(
            message in prop::array::uniform32(u8::MIN..u8::MAX)
        ) {
            let poly = Poly::read_msg(&message).unwrap();
            let comp_message = poly.normalise().write_msg().unwrap();

            assert_eq!(message, comp_message);
        }

    }
}
