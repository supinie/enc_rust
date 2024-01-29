#[cfg(test)]
mod matrix_tests {
    use crate::{
        matrix::*,
        params::{SecurityLevel, K, N},
        polynomials::Poly,
        tests::sample::sample_tests::{generate_random_seed, uniform_dist_test},
        vectors::{PolyVec1024, PolyVec512, PolyVec768},
    };

    static TEST_PARAMS: [SecurityLevel; 3] = [
        SecurityLevel::new(K::Two),
        SecurityLevel::new(K::Three),
        SecurityLevel::new(K::Four),
    ];

    #[test]
    fn derive_test() {
        let seed = generate_random_seed();
        for sec_level in &TEST_PARAMS {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let matrix = Mat512::derive(&seed, false);
                let mut matrix_t = Mat512::derive(&seed, true);
                matrix_t.transpose();
                assert_eq!(matrix, matrix_t);
                for poly_vec in &matrix {
                    for poly in poly_vec {
                        uniform_dist_test(poly);
                    }
                }
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let matrix = Mat768::derive(&seed, false);
                let mut matrix_t = Mat768::derive(&seed, true);
                matrix_t.transpose();
                assert_eq!(matrix, matrix_t);
                for poly_vec in &matrix {
                    for poly in poly_vec {
                        uniform_dist_test(poly);
                    }
                }
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let matrix = Mat1024::derive(&seed, false);
                let mut matrix_t = Mat1024::derive(&seed, true);
                matrix_t.transpose();
                assert_eq!(matrix, matrix_t);
                for poly_vec in &matrix {
                    for poly in poly_vec {
                        uniform_dist_test(poly);
                    }
                }
            }
        }
    }

    #[test]
    fn transpose_test() {
        for sec_level in &TEST_PARAMS {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut matrix: Mat512 =
                    [PolyVec512::from([Poly { coeffs: [1; N] }, Poly { coeffs: [2; N] }]); 2];
                matrix.transpose();
                assert_eq!(
                    matrix,
                    [
                        PolyVec512::from([Poly { coeffs: [1; N] }; 2]),
                        PolyVec512::from([Poly { coeffs: [2; N] }; 2])
                    ]
                );
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut matrix: Mat768 = [PolyVec768::from([
                    Poly { coeffs: [1; N] },
                    Poly { coeffs: [2; N] },
                    Poly { coeffs: [3; N] },
                ]); 3];
                matrix.transpose();
                assert_eq!(
                    matrix,
                    [
                        PolyVec768::from([Poly { coeffs: [1; N] }; 3]),
                        PolyVec768::from([Poly { coeffs: [2; N] }; 3]),
                        PolyVec768::from([Poly { coeffs: [3; N] }; 3])
                    ]
                );
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut matrix: Mat1024 = [PolyVec1024::from([
                    Poly { coeffs: [1; N] },
                    Poly { coeffs: [2; N] },
                    Poly { coeffs: [3; N] },
                    Poly { coeffs: [4; N] },
                ]); 4];
                matrix.transpose();
                assert_eq!(
                    matrix,
                    [
                        PolyVec1024::from([Poly { coeffs: [1; N] }; 4]),
                        PolyVec1024::from([Poly { coeffs: [2; N] }; 4]),
                        PolyVec1024::from([Poly { coeffs: [3; N] }; 4]),
                        PolyVec1024::from([Poly { coeffs: [4; N] }; 4])
                    ]
                );
            }
        }
    }
}
