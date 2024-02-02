#![allow(warnings)]
#[cfg(test)]



mod field_tests {
    use crate::{field_operations::*, params::Q};
    use proptest::prelude::*;

    const montgomery_reduce_limit: i32 = (Q as i32) * (2 as i32).pow(15);
    proptest! {
        #[test]
        fn montgomery_reduce_test(i in -montgomery_reduce_limit..montgomery_reduce_limit) {
            montgomery_reduce(i);
        }

        #[test] 
        fn mont_form_test(i: i16) {
            mont_form(i);
        }

        #[test]
        fn barrett_reduce_test(i: i16) {
            barrett_reduce(i);
        }

        #[test]
        fn conditional_sub_q_test(i: i16) {
            conditional_sub_q(i);
        }
    }
}
