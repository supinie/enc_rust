#[cfg(test)]
mod buffer_tests {
    use crate::{buffer::*, params::*, poly::*};
    use rand::Rng;

    impl Buffer {
        pub fn generate_random(size: usize) -> Buffer {
            let mut rng = rand::thread_rng();
            let mut data = Vec::with_capacity(size);

            for _ in 0..size {
                data.push(rng.gen::<u8>());
            }

            Buffer { data, pointer: 0 }
        }
    }

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

        let _bad_result = buffer.read(6);
    }

    #[test]
    fn reset_test() {
        let mut buffer = Buffer::new();
        buffer.pointer = 3;
        buffer.reset();
        assert_eq!(buffer.pointer, 0);
    }

    #[test]
    fn pack_unpack_test() {
        let p = Poly { coeffs: [20; N] };
        let mut buffer = Buffer::zero_initialise(3 * 128);
        buffer.pack(p);

        let mut comp_p = Poly::new();
        comp_p.unpack(buffer);

        assert_eq!(comp_p.coeffs, p.coeffs);
    }

    #[test]
    fn compress_decompress_test() {
        let buf1 = Buffer::generate_random(Params::sec_level_512().poly_compressed_bytes());
        let buf2 = Buffer::generate_random(Params::sec_level_768().poly_compressed_bytes());
        let buf3 = Buffer::generate_random(Params::sec_level_1024().poly_compressed_bytes());
        let mut buf_comp1 = Buffer::zero_initialise(Params::sec_level_512().poly_compressed_bytes());
        let mut buf_comp2 = Buffer::zero_initialise(Params::sec_level_768().poly_compressed_bytes());
        let mut buf_comp3 = Buffer::zero_initialise(Params::sec_level_1024().poly_compressed_bytes());

        let mut poly1 = Poly::new();
        let mut poly2 = Poly::new();
        let mut poly3 = Poly::new();

        poly1.decompress(&buf1, Params::sec_level_512().poly_compressed_bytes());
        poly2.decompress(&buf2, Params::sec_level_768().poly_compressed_bytes());
        poly3.decompress(&buf3, Params::sec_level_1024().poly_compressed_bytes());
        buf_comp1.compress(poly1, Params::sec_level_512().poly_compressed_bytes());
        buf_comp2.compress(poly2, Params::sec_level_768().poly_compressed_bytes());
        buf_comp3.compress(poly3, Params::sec_level_1024().poly_compressed_bytes());

        assert_eq!(buf_comp1.data, buf1.data);
        assert_eq!(buf_comp2.data, buf2.data);
        assert_eq!(buf_comp3.data, buf3.data);
    }
}
