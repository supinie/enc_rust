#![allow(warnings)]
#[cfg(test)]

mod poly_tests {
    use crate::{polynomials::*, params::*};
    use proptest::prelude::*;

    pub(in crate::tests) fn new_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(-(Q as i16)..(Q as i16))
    }

    #[test]
    fn new_test() {
        let poly = Poly::new();
    }

    proptest! {
        #[test]
        fn from_arr_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);
        }

        #[test]
        fn pointwise_mul_test(
            a in new_poly_array(),
            b in new_poly_array()
        ) {
            let mut poly_a = Poly::from_arr(&a).normalise();
            let poly_b = Poly::from_arr(&b).normalise();

            poly_a.pointwise_mul(&poly_b);
        }

        #[test]
        fn write_msg_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a).normalise();
            let msg = poly.write_msg().unwrap();
        }

        #[test]
        fn read_msg_test(msg in prop::array::uniform32(0u8..)) {
            let poly = Poly::read_msg(&msg).unwrap();
        }
    }
}
