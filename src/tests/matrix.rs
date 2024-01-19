#[cfg(test)]
mod matrix_tests {
    use crate::{
        matrix::*,
        params::{SecurityLevel, K, N}, polynomials::Poly, vectors::PolyVec512,
    };

    static TEST_PARAMS: [SecurityLevel; 3] = [
        SecurityLevel::new(K::Two),
        SecurityLevel::new(K::Three),
        SecurityLevel::new(K::Four),
    ];


    // #[test]
    // fn transpose_test() {
        // let mut matrix: Mat512 = [PolyVec512::from([Poly { coeffs: [1; N] }, Poly {coeffs: [2; N] }]); 2];
        // matrix.transpose();
        // assert_eq!(matrix, [PolyVec512::from([Poly { coeffs: [1; N]}; 2]), PolyVec512::from([Poly { coeffs: [2; N]}; 2])]);
    // }
}
