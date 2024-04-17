#![allow(warnings)]
#[cfg(test)]

mod matrix_tests {
    use crate::{
        matrix::*, params::*, polynomials::Montgomery,
        tests::params::params_tests::sec_level_strategy,
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
