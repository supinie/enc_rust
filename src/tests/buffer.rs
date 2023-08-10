#[cfg(test)]
mod buffer_tests {
    use crate::buffer::*;

    #[test]
    fn new_test() {
        let buffer = Buffer::new();
        assert_eq!(buffer.pointer, 0);
        assert_eq!(buffer.data.len(), 0);
    }

    #[test]
    fn push_and_valid_read_test() {
        let mut buffer = Buffer::new();
        buffer.push(&[1, 2, 3, 4, 5]);
        
        let good_result = buffer.read(3);
        assert_eq!(good_result, &[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn push_and_invalid_read_test() {
        let mut buffer = Buffer::new();
        buffer.push(&[1, 2, 3, 4, 5]);
        
        let bad_result = buffer.read(6);
    }

    #[test]
    fn reset_test() {
        let mut buffer = Buffer::new();
        buffer.pointer = 3;
        buffer.reset();
        assert_eq!(buffer.pointer, 0);
    }
}
