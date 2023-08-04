#[cfg(test)]
mod poly_tests {
    use crate::{params::*, poly::*};    
    use more_asserts::{assert_gt, assert_lt};

    // Test Poly::new()
    #[test]
    fn new_test() {
        let poly = Poly::new();
        assert_eq!(poly.coeffs, [0; N]);
    }

    // Test Poly::add()
    #[test]
    fn add_test() {
        let mut poly1 = Poly { coeffs: [1; N] };
        let poly2 = Poly { coeffs: [4; N] };
        poly1.add(&poly2);
        assert_eq!(poly1.coeffs, [5; N]);
    }

    // Test Poly::sub()
    #[test]
    fn sub_test() {
        let mut poly1 = Poly { coeffs: [3; N] };
        let poly2 = Poly { coeffs: [1; N] };
        poly1.sub(&poly2);
        assert_eq!(poly1.coeffs, [2; N]);
    }

    // // Test Poly::ntt()
    // #[test]
    // fn poly_ntt_test() {
    //     // Write a test case to check if the NTT function is working as expected.
    //     // You might need to add additional helper functions if required.
    // }

    // // Test Poly::inv_ntt()
    // #[test]
    // fn poly_inv_ntt_test() {
    //     // Write a test case to check if the inverse NTT function is working as expected.
    //     // You might need to add additional helper functions if required.
    // }

    // Test Poly::ntt() and Poly::inv_ntt()
    #[test]
    fn ntt_inv_ntt_test() {
        // Create a random input polynomial.
        let mut input_poly = Poly { coeffs: [20; N] };

        // Save a copy of the original input for later comparison.
        let mut original_input = input_poly;

        original_input.normalise();

        // Perform NTT
        input_poly.ntt();

        for coefficient in input_poly.coeffs {
            assert_lt!(coefficient, 7*(Q as i16));
            assert_gt!(coefficient, -7*(Q as i16));
        }
            

        input_poly.normalise();

        // Perform inverse NTT
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
        // // The result of inverse NTT should match the original input.
        // assert_eq!(input_poly.coeffs, original_input.coeffs);
    }
}
