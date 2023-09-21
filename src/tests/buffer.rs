#[cfg(test)]
mod buffer_tests {
    use crate::{buffer::*, params::*, poly::*};
    use rand::Rng;
 
    static TEST_PARAMS: [Params; 3] = [
        Params::sec_level_512(),
        Params::sec_level_768(),
        Params::sec_level_1024(),
    ];

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
        for sec_level in TEST_PARAMS.iter() {
            let buf = Buffer::generate_random(sec_level.poly_compressed_bytes());
            let mut buf_comp = Buffer::zero_initialise(sec_level.poly_compressed_bytes());

            let mut poly = Poly::new();

            poly.decompress(&buf, sec_level.poly_compressed_bytes());
            buf_comp.compress(poly, sec_level.poly_compressed_bytes());

            assert_eq!(buf_comp.data, buf.data);
            assert_eq!(buf_comp.data, buf.data);
            assert_eq!(buf_comp.data, buf.data);
        }
    }
}
