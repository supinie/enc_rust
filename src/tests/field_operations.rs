#![allow(warnings)]
#[cfg(test)]

mod field_tests {
    use crate::{field_operations::*, params::Q};
    use proptest::prelude::*;

    const MONTGOMERY_REDUCE_LIMIT: i32 = (Q as i32) * (2 as i32).pow(15);

    fn modQ(x: i32) -> i16 {
        let mut y = (x % Q as i32) as i16;
        if y < 0 {
            y += Q as i16;
        }
        y
    }

    proptest! {
        #[test]
        fn montgomery_reduce_test(i in -MONTGOMERY_REDUCE_LIMIT..MONTGOMERY_REDUCE_LIMIT) {
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
        fn montgomery_reduce_test_alt(x in -(Q as i32) * (1 << 15)..(Q as i32) * (1 << 15)) {
            let y = montgomery_reduce(x);

            assert_eq!(modQ(x), modQ((y as i32) * (1 << 16)));
        }


        #[test]
        fn mont_form_test(i: i16) {
            let output = mont_form(i);
            
            assert_eq!(modQ(output as i32), modQ(i as i32 * 2285));
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
        fn barrett_reduce_test_alt(i: i16) {
            let mut output = barrett_reduce(i);
            let mut y = i % Q as i16;

            if y < 0 {
                y += Q as i16;
            }
            if output < 0 && (-output) % Q as i16 == 0 {
                output -= Q as i16;
            }

            assert_eq!(output, y);
        }

        #[test]
        fn conditional_sub_q_test(i: i16) {
            let output = conditional_sub_q(i);

            let mut y = i as i32;
            if i >= Q as i16 {
                y -= Q as i32;
            }
            assert_eq!(output, y as i16);
        }
    }
}
