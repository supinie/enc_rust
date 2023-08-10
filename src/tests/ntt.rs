#[cfg(test)]
mod ntt_tests {
    use crate::{params::*, poly::*};
    use more_asserts::{assert_gt, assert_lt};

    // Test Poly::ntt() and Poly::inv_ntt()
    #[test]
    fn ntt_inv_ntt_test() {
        let mut input_poly = Poly { coeffs: [20; N] };

        // Save a copy of the original input for later comparison.
        let mut original_input = input_poly;
        original_input.normalise();

        input_poly.ntt();

        for coefficient in input_poly.coeffs {
            assert_lt!(coefficient, 7*(Q as i16));
            assert_gt!(coefficient, -7*(Q as i16));
        }
            

        input_poly.normalise();
        input_poly.inv_ntt();

        for coefficient in input_poly.coeffs {
            assert_lt!(coefficient, (Q as i16));
            assert_gt!(coefficient, -(Q as i16));
        }

        input_poly.normalise();

        for i in 0..N {
            let p: i32 = input_poly.coeffs[i] as i32;
            let q: i32 = original_input.coeffs[i] as i32;
            assert_eq!(p, (q * (1<<16))%(Q as i32), "we are testing equality with original at index {}", i);
        }
    }
}
