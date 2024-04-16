#![allow(warnings)]
#[cfg(test)]

mod vec_tests {
    use crate::{
        vectors::*,
        polynomials::*,
        params::*,
        tests::polynomials::poly_tests::{
            new_poly,
            new_limited_poly,
            new_ntt_poly,
        },
        tests::params::params_tests::sec_level_strategy,
    };
    use proptest::prelude::*;
    use tinyvec::ArrayVec;

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
        }

        #[test]
        fn decompress_test(poly_vec in new_poly_vec()) {
            let mut buf = [0u8; 4 * 160]; // max poly_vec_compressed_bytes
            let end = poly_vec.sec_level().poly_vec_compressed_bytes();

            let _result = poly_vec.normalise().compress(&mut buf[..end]).unwrap();

            let decompressed = PolyVec::decompress(&buf[..end]).unwrap();
        }

        #[test]
        fn derive_noise_test(
            sec_level in sec_level_strategy(),
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            nonce in (u8::MIN..u8::MAX),
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
