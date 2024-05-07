#![allow(warnings)]
#[cfg(test)]

mod vec_tests {
    use crate::{
        params::*,
        polynomials::*,
        tests::params::params_tests::sec_level_strategy,
        tests::polynomials::poly_tests::{new_limited_poly, new_ntt_poly, new_poly},
        vectors::*,
    };
    use proptest::prelude::*;
    use tinyvec::{array_vec, ArrayVec};

    prop_compose! {
        pub(in crate::tests) fn new_poly_vec()
            (sec_level in 2..=4usize, poly_1 in new_poly(), poly_2 in new_poly(), poly_3 in new_poly(), poly_4 in new_poly())
            -> PolyVec<Unreduced> {
                PolyVec::from(ArrayVec::<[Poly<Unreduced>; 4]>::from_array_len([poly_1, poly_2, poly_3, poly_4], sec_level)).unwrap()
            }
    }

    prop_compose! {
        // restrict sec_level here so that we can ensure two polyvecs are the same sec_level
        pub(in crate::tests) fn new_limited_poly_vec(sec_level: usize)
            (poly_1 in new_limited_poly(), poly_2 in new_limited_poly(), poly_3 in new_limited_poly(), poly_4 in new_limited_poly())
            -> PolyVec<Unreduced> {
                PolyVec::from(ArrayVec::<[Poly<Unreduced>; 4]>::from_array_len([poly_1, poly_2, poly_3, poly_4], sec_level)).unwrap()
            }
    }

    prop_compose! {
        // restrict sec_level here so that we can ensure two polyvecs are the same sec_level
        pub(in crate::tests) fn new_ntt_poly_vec(sec_level: usize)
            (poly_1 in new_ntt_poly(), poly_2 in new_ntt_poly(), poly_3 in new_ntt_poly(), poly_4 in new_ntt_poly())
            -> PolyVec<Unreduced> {
                PolyVec::from(ArrayVec::<[Poly<Unreduced>; 4]>::from_array_len([poly_1, poly_2, poly_3, poly_4], sec_level)).unwrap()
            }
    }

    #[test]
    #[should_panic]
    fn from_test() {
        // new should be empty so will fail
        let err_result = PolyVec::from(ArrayVec::<[Poly<Unreduced>; 4]>::new()).unwrap();
    }

    #[test]
    fn compare_inner_product_pointwise() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).mont_form();
        let v = PolyVec::from(array_vec!([Poly<Montgomery>; 4] => poly, poly, poly)).unwrap();
        let w = PolyVec::from(array_vec!([Poly<Montgomery>; 4] => poly, poly, poly)).unwrap();

        let output = v.inner_product_pointwise(&w);
        let want = [-333, 0, 5856, -1410, -1209, 8616, -5412, -9870, -5115, 3054, 2823, 7440, 7059, 3288, -627, -9402, 4089, 9318, -1950, -474, -1545, 1170, -240, -5724, 5175, -1182, 3267, -5178, -2979, 2262, 8367, 1164, 1053, -8472, -2406, -6672, 801, 6564, -1296, -8712, -1728, 7422, 525, -4956, -1044, -5898, 8322, 4596, 471, 6552, -3213, -30, -8382, 4824, 1023, 1140, 2262, 8892, -4452, 8106, -6075, -1218, 2961, 894, -462, -5532, -4776, -522, 3768, -4050, -2358, 3858, -4722, 3228, -1764, -5940, -489, -3672, 8559, -9942, -2724, -4776, 4683, -8148, 2658, -84, 3609, -558, -1050, -9570, 1035, -7146, -5886, 6714, 6675, -7938, -9081, 8820, 4830, -2934, -66, -3252, -7635, 7866, 5187, -9528, 3033, 4488, -1530, -10008, -732, 6906, 2226, -4692, 1509, -4854, -2994, 6420, -2937, 9156, -6090, 3354, 4533, 8988, 8142, 6084, -2274, -5358, -8832, -5364, 7920, 6066, -2583, 8958, -210, 3312, 1386, 9102, -2943, 6354, -312, -4932, -2166, -4782, 5373, 6804, -3588, 9852, -3501, 4362, 996, -9666, -3084, 7716, 5682, -3414, -165, -3108, -6738, 8634, -2304, -8136, -1782, 6504, -5442, -7368, 3117, -9804, -5019, -804, 1989, -342, 6039, -8418, 2622, -5058, -3522, 9738, -7677, -3978, -1206, -6258, -4140, 2898, -3501, 3516, 1593, -4404, 531, -888, -399, -5910, 6546, 504, -177, -1620, -1095, 7692, -3987, 8466, -6381, 702, 3513, 4374, 5079, -492, -4086, 6078, 4353, 4110, 1290, -6396, -7218, -5466, -114, 6900, 2592, -9246, 6780, 6018, 1383, -7230, 4707, -9042, -2898, 582, 5898, 1668, -78, -5784, 1242, -1800, 447, -6354, -1068, 528, 2043, -1128, 1257, 8652, 579, 9894, 2841, 2598, 6, 6738, -4716, 2340, 780, 9378, -690, 7878, 2829, -2160, -2184, -762];
        assert_eq!(output.coeffs(), &want);
    }

    proptest! {
        #[test]
        fn sec_level_test(poly_vec in new_poly_vec()) {
            let sec_level = poly_vec.sec_level();
        }

        #[test]
        fn polynomials_test(poly_vec in new_poly_vec()) {
            let polys = poly_vec.polynomials();
        }

        #[test]
        fn add_test(a in new_limited_poly_vec(4), b in new_limited_poly_vec(4)) {
            let poly_vec = a.add(&b).unwrap();
        }

        #[test]
        #[should_panic]
        fn add_test_different_sec_levels(a in new_limited_poly_vec(2), b in new_limited_poly_vec(3)) {
            let poly_vec = a.add(&b).unwrap();
        }

        #[test]
        fn barrett_reduce_test(poly_vec in new_poly_vec()) {
            let output = poly_vec.barrett_reduce();
        }

        #[test]
        fn normalise_test(poly_vec in new_poly_vec()) {
            let output = poly_vec.normalise();
        }

        #[test]
        fn inv_ntt_test(poly_vec in new_ntt_poly_vec(4)) {
            let output = poly_vec.inv_ntt();
        }

        #[test]
        fn ntt_test(poly_vec in new_poly_vec()) {
            let output = poly_vec.normalise().ntt();
        }

        #[test]
        fn new_test(sec_level in sec_level_strategy()) {
            let output = PolyVec::new(sec_level.k());
        }

        #[test]
        fn pack_test(poly_vec in new_poly_vec()) {
            let mut buf = [0u8; 4 * POLYBYTES]; // max buf length needed
            let k: usize = poly_vec.sec_level().k().into();

            let output = poly_vec.normalise().pack(&mut buf[..k * POLYBYTES]).unwrap();
        }

        #[test]
        fn compress_test(poly_vec in new_poly_vec()) {
            let mut buf = [0u8; 4 * 160]; // max poly_vec_compressed_bytes
            let end = poly_vec.sec_level().poly_vec_compressed_bytes();

            let output = poly_vec.normalise().compress(&mut buf[..end]).unwrap();
        }

        #[test]
        fn unpack_test(poly_vec in new_poly_vec()) {
            let mut buf = [0u8; 4 * POLYBYTES]; // max buf length
            let k: usize = poly_vec.sec_level().k().into();

            let _result = poly_vec.normalise().pack(&mut buf[..k * POLYBYTES]).unwrap();

            let unpacked = PolyVec::unpack(&buf[..k * POLYBYTES]).unwrap();
            assert_eq!(unpacked.normalise(), poly_vec.normalise());
        }

        #[test]
        fn decompress_test(poly_vec in new_poly_vec()) {
            let mut buf = [0u8; 4 * 160]; // max poly_vec_compressed_bytes
            let end = poly_vec.sec_level().poly_vec_compressed_bytes();

            let _result = poly_vec.normalise().compress(&mut buf[..end]).unwrap();

            let decompressed = PolyVec::decompress(&buf[..end]).unwrap();
            // assert_eq!(poly_vec.normalise(), decompressed);
        }

        #[test]
        fn derive_noise_test(
            sec_level in sec_level_strategy(),
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            nonce in (u8::MIN..u8::MAX - 4),
        ) {
            let output_1 = PolyVec::derive_noise(sec_level, &seed, nonce, sec_level.eta_1());
            let output_2 = PolyVec::derive_noise(sec_level, &seed, nonce, sec_level.eta_2());
        }

        #[test]
        fn inner_product_pointwise_test(
            poly_vec_1 in new_limited_poly_vec(4),
            poly_vec_2 in new_limited_poly_vec(4),
        ) {
            let poly = poly_vec_1.normalise().inner_product_pointwise(&poly_vec_2.normalise());
            let poly = poly_vec_1.barrett_reduce().inner_product_pointwise(&poly_vec_2.barrett_reduce());
        }
    }
}
