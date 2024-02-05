#![allow(warnings)]
#[cfg(test)]
mod poly_tests {
    use crate::{
        field_operations::{barrett_reduce, montgomery_reduce},
        params::*,
        polynomials::*,
        ntt::ZETAS,
        tests::buffer::buffer_tests::zero_initialise_buffer,    
    };
    use proptest::prelude::*;

    impl Poly {
        fn pointwise_mul_alt(&mut self, x: &Self) {
            let mut j: usize = 64;

            for i in (0..N).step_by(4) {
                let zeta = i32::from(ZETAS[j]);
                j += 1;

                let mut p0 =
                    montgomery_reduce(i32::from(self.coeffs[i + 1]) * i32::from(x.coeffs[i + 1]));
                p0 = montgomery_reduce(i32::from(p0) * zeta);
                p0 += montgomery_reduce(i32::from(self.coeffs[i]) * i32::from(x.coeffs[i]));

                let mut p1 = montgomery_reduce(i32::from(self.coeffs[i]) * i32::from(x.coeffs[i + 1]));
                p1 += montgomery_reduce(i32::from(self.coeffs[i + 1]) * i32::from(x.coeffs[i]));

                let mut p2 =
                    montgomery_reduce(i32::from(self.coeffs[i + 3]) * i32::from(x.coeffs[i + 3]));
                p2 = -montgomery_reduce(i32::from(p2) * zeta);
                p2 += montgomery_reduce(i32::from(self.coeffs[i + 2]) * i32::from(x.coeffs[i + 2]));

                let mut p3 =
                    montgomery_reduce(i32::from(self.coeffs[i + 2]) * i32::from(x.coeffs[i + 3]));
                p3 += montgomery_reduce(i32::from(self.coeffs[i + 3]) * i32::from(x.coeffs[i + 2]));

                self.coeffs[i] = p0;
                self.coeffs[i + 1] = p1;
                self.coeffs[i + 2] = p2;
                self.coeffs[i + 3] = p3;
            }
        }
    }

    // Test Poly::new()
    #[test]
    fn new_test() {
        let poly = Poly::new();
        assert_eq!(poly.coeffs, [0; N]);
    }

    proptest! {
        #[test]
        fn pointwise_mul_test(
            a in prop::array::uniform(-(Q as i16)..(Q as i16)),
            b in prop::array::uniform(-(Q as i16)..(Q as i16))
        ) {
            let mut poly_a = Poly { coeffs: a };
            let poly_b = Poly { coeffs: b };

            let mut a_copy = poly_a;
            let b_copy = poly_b;

            poly_a.pointwise_mul(&poly_b);
            a_copy.pointwise_mul_alt(&poly_b);

            assert_eq!(poly_a, a_copy);
        }

        #[test]
        fn to_and_from_msg_test(a in prop::array::uniform(-(Q as i16)..(Q as i16))) {
            let mut poly = Poly { coeffs: a};
            poly.normalise();
            let mut msg = zero_initialise_buffer(32);

            poly.write_msg(&mut msg);
            
            let mut new_poly = Poly::new();
            new_poly.read_msg(&msg);

            for (&coeff, new_coeff) in poly.coeffs.iter().zip(new_poly.coeffs) {
                if 833 <= coeff && coeff <= 2496 {
                    assert_eq!(new_coeff, (Q as i16 + 1) / 2, "{}", coeff);
                } else {
                    assert_eq!(new_coeff, 0, "{}", coeff);
                }
            }
        }

        #[test]
        fn add_test(
            a in prop::array::uniform(-(Q as i16)..(Q as i16)),
            b in prop::array::uniform(-(Q as i16)..(Q as i16))
        ) {
            let mut poly_a = Poly { coeffs: a };
            let poly_b = Poly { coeffs: b };

            poly_a.add(&poly_b);
        }

        #[test]
        fn sub_test(
            a in prop::array::uniform(-(Q as i16)..(Q as i16)),
            b in prop::array::uniform(-(Q as i16)..(Q as i16))
        ) {
            let mut poly_a = Poly { coeffs: a };
            let poly_b = Poly { coeffs: b };

            poly_a.sub(&poly_b);
        }

        #[test]
        fn mont_form_test(a in prop::array::uniform(-(Q as i16)..(Q as i16))) {
            let mut poly = Poly { coeffs: a };
            poly.mont_form();
        }
    }
}
