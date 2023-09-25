#[cfg(test)]
mod sample_tests {
    use crate::{params::*, poly::*};
    use std::ops::Range;

    #[test]
    fn derive_noise_2_test() {
        let range: Range<i16> = -2..3;
        let mut poly = Poly::new();
        poly.derive_noise_2(&[8], 8);
        for coeff in poly.coeffs.iter() {
            println!("{}", coeff);
            assert!(range.contains(coeff), "coefficient {} not in valid range", coeff);
        }
    }


    #[test]
    fn derive_noise_3_test() {
        let range: Range<i16> = -3..4;
        let mut poly = Poly::new();
        poly.derive_noise_3(&[8], 8);
        for coeff in poly.coeffs.iter() {
            println!("{}", coeff);
            assert!(range.contains(coeff), "coefficient {} not in valid range", coeff);
        }
    }
}
