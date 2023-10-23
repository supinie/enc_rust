#[cfg(test)]
mod field_tests {
    use crate::field_ops::*;

    #[test]
    pub(in crate::tests) fn montgomery_reduce_test() {
        assert_eq!(montgomery_reduce(i32::MAX), 32599);
        assert_eq!(montgomery_reduce(i32::MIN), -32768);
    }

    #[test]
    pub(in crate::tests) fn mont_form_test() {
        assert_eq!(mont_form(i16::MAX), 56);
        assert_eq!(mont_form(i16::MIN), 988);
    }

    #[test]
    pub(in crate::tests) fn barrett_reduce_test() {
        assert_eq!(barrett_reduce(i16::MAX), 2806);
        assert_eq!(barrett_reduce(i16::MIN), 522);
    }

    #[test]
    pub(in crate::tests) fn cond_sub_q_test() {
        assert_eq!(cond_sub_q(i16::MAX), 29438);
        assert_eq!(cond_sub_q(-29439), -29439);
    }
}
