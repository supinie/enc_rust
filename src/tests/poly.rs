#[cfg(test)]
mod poly_tests {
    use crate::{params::*, poly::*, field_ops::{montgomery_reduce, barrett_reduce}};    
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

    // Test Poly::pointwise_mul()
    #[test]
    fn pointwise_mul_test() {
        let mut a = Poly { coeffs: [Q as i16; N] };
        let mut b = Poly { coeffs: [20; N] };
        let mut p = Poly::new();

        b.coeffs[0] = 1;
        
        let mut a_copy = a;
        let mut b_copy = b;

        a_copy.ntt();
        b_copy.ntt();

        a_copy.pointwise_mul(&b_copy);
        a_copy.barrett_reduce();
        a_copy.inv_ntt();

        for i in 0..N {
            for j in 0..N {
                let mut v = montgomery_reduce((a.coeffs[i] as i32) * (b.coeffs[j] as i32));
                let mut k = i + j;
                
                // circular shifting case; x^N = -1
                if k >= N {
                    k -= N;
                    v = -v;
                }
                p.coeffs[k] = barrett_reduce(v + p.coeffs[k]);
            }
        }

        for i in 0..N {
            p.coeffs[i] = (((p.coeffs[i] as i32) * ((1 << 16) % (Q as i32))) % (Q as i32)) as i16;
        }

        p.normalise();
        a_copy.normalise();

        assert_eq!(p.coeffs, a_copy.coeffs);
    }
}
