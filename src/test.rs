#[cfg(test)]
mod tests {
    use crate::{params::*, field_ops::*, poly::*};    
    use more_asserts::{assert_gt, assert_lt};

    // Test Poly::new()
    #[test]
    fn test_poly_new() {
        let poly = Poly::new();
        assert_eq!(poly.coeffs, [0; N]);
    }

    // Test Poly::add()
    #[test]
    fn test_poly_add() {
        let mut poly1 = Poly { coeffs: [1; N] };
        let poly2 = Poly { coeffs: [4; N] };
        poly1.add(&poly2);
        assert_eq!(poly1.coeffs, [5; N]);
    }

    // Test Poly::sub()
    #[test]
    fn test_poly_sub() {
        let mut poly1 = Poly { coeffs: [3; N] };
        let poly2 = Poly { coeffs: [1; N] };
        poly1.sub(&poly2);
        assert_eq!(poly1.coeffs, [2; N]);
    }

    // // Test Poly::ntt()
    // #[test]
    // fn test_poly_ntt() {
    //     // Write a test case to check if the NTT function is working as expected.
    //     // You might need to add additional helper functions if required.
    // }

    // // Test Poly::inv_ntt()
    // #[test]
    // fn test_poly_inv_ntt() {
    //     // Write a test case to check if the inverse NTT function is working as expected.
    //     // You might need to add additional helper functions if required.
    // }

    // Test Poly::ntt() and Poly::inv_ntt()
    #[test]
    fn test_ntt_inv_ntt() {
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

    #[test]
    pub fn test_montgomery_reduce() {
        assert_eq!(montgomery_reduce(i32::MAX), 32599);
        assert_eq!(montgomery_reduce(i32::MIN), -32768);
    }

    #[test]
    pub fn test_to_mont() {
        assert_eq!(to_mont(i16::MAX), 56);
        assert_eq!(to_mont(i16::MIN), 988);
    }

    #[test]
    pub fn test_barrett_reduce() {
        assert_eq!(barrett_reduce(i16::MAX), 2806);
        assert_eq!(barrett_reduce(i16::MIN), 522);
    }

    #[test]
    pub fn test_cond_sub_q() {
        assert_eq!(cond_sub_q(i16::MAX), 29438);
        assert_eq!(cond_sub_q(-29439), -29439);
    }
}
