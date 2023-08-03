#[cfg(test)]
mod tests {
    use crate::{params::*, field_ops::*, poly::*};    
    use more_asserts as ma;

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
        println!("{:?}", input_poly.coeffs[0]);

        for coefficient in input_poly.coeffs {
            ma::assert_lt!(coefficient, 7*(Q as i16));
            ma::assert_gt!(coefficient, -7*(Q as i16));
        }
            

        input_poly.normalise();
        println!("{:?}", input_poly.coeffs[0]);

        // Perform inverse NTT
        input_poly.inv_ntt();
        println!("{:?}", input_poly.coeffs[0]);

        for coefficient in input_poly.coeffs {
            ma::assert_lt!(coefficient, (Q as i16));
            ma::assert_gt!(coefficient, -(Q as i16));
        }

        input_poly.normalise();
        println!("{:?}", input_poly.coeffs[0]);

        for i in 0..N {
            let p: i32 = input_poly.coeffs[i] as i32;
            let q: i32 = original_input.coeffs[i] as i32;
            assert_eq!(p, (q * (1<<16))%(Q as i32), "we are testing equality with original at index {}", i);
        }
        // // The result of inverse NTT should match the original input.
        // assert_eq!(input_poly.coeffs, original_input.coeffs);
    }
}
