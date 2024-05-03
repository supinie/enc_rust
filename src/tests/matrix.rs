#![allow(warnings)]
#[cfg(test)]

mod matrix_tests {
    use crate::{
        matrix::*, params::*, polynomials::Montgomery,
        tests::params::params_tests::sec_level_strategy,
        tests::indcpa::indcpa_tests::*,
    };
    use proptest::prelude::*;
    use tinyvec::ArrayVec;

    prop_compose! {
        pub(in crate::tests) fn new_matrix()
            (sec_level in sec_level_strategy(), seed in prop::array::uniform32(u8::MIN..u8::MAX), transpose in prop::bool::ANY)
            -> Matrix<Montgomery> {
                Matrix::derive(&seed, transpose, sec_level.k()).unwrap()
            }
    }

    #[test]
    fn derive_seed_test() {
        let seed: [u8; 32] = [216, 17, 145, 112, 104, 44, 220, 160, 102, 24, 217, 187, 231, 175, 1, 61, 77, 228, 144, 197, 40, 188, 178, 237, 151, 66, 203, 184, 231, 204, 11, 173];

        let matrix = Matrix::derive(&seed, true, K::Three).unwrap();

        // assert_eq!(matrix, Matrix::test_mat());
    }

    proptest! {
        #[test]
        fn derive_test(
            sec_level in sec_level_strategy(),
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            transpose in prop::bool::ANY,
        ) {
            let matrix = Matrix::derive(&seed, transpose, sec_level.k()).unwrap();
        }

        #[test]
        fn sec_level_test(mat in new_matrix()) {
            let sec_level = mat.sec_level();
        }

        #[test]
        fn vectors_test(mat in new_matrix()) {
            let vecs = mat.vectors();
        }

        #[test]
        fn transpose_test(
            sec_level in sec_level_strategy(),
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
        ) {
            let matrix_1 = Matrix::derive(&seed, false, sec_level.k()).unwrap().transpose().unwrap();
            let matrix_2 = Matrix::derive(&seed, true, sec_level.k()).unwrap();

            assert_eq!(matrix_1, matrix_2);
        }
    }
}
