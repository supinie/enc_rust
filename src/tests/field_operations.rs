#![allow(warnings)]
#[cfg(test)]

mod field_tests {
    use crate::{field_operations::*, params::Q};
    use proptest::prelude::*;

    const montgomery_reduce_limit: i32 = (Q as i32) * (2 as i32).pow(15);
    proptest! {
        #[test]
        fn montgomery_reduce_test(i in -montgomery_reduce_limit..montgomery_reduce_limit) {
            let output_1 = montgomery_reduce(i);

            let ua = i.wrapping_mul(62209) as i16;
            let u = ua as i32;
            let mut t = u * Q as i32;
            t = i - t;
            t >>= 16;
            let output_2 = t as i16;

            assert_eq!(output_1, output_2);
        }

        #[test]
        fn mont_form_test(i: i16) {
            let output = mont_form(i);
        }

        #[test]
        fn barrett_reduce_test(i in -(Q as i16)..(Q as i16)) {
            let output = barrett_reduce(i);

            let v = ((1u32 << 26) / Q as u32 + 1) as i32;
            let mut t = v * i as i32 + (1 << 25);
            t >>= 26;
            t *= Q as i32;
            let output_2 = i - t as i16;

            assert_eq!(output.rem_euclid(Q as i16), output_2.rem_euclid(Q as i16));
        }

        #[test]
        fn conditional_sub_q_test(i: i16) {
            conditional_sub_q(i);
        }
    }
}
