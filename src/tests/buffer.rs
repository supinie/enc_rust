#[cfg(test)]
mod buffer_tests {
    use crate::{buffer::*, params::*, poly::*};
    use rand::Rng;
    extern crate std;
    use std::vec::Vec;
 
    static TEST_PARAMS: [Params; 3] = [
        Params::sec_level_512(),
        Params::sec_level_768(),
        Params::sec_level_1024(),
    ];

    fn zero_initialise_buffer(size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(0u8);
        }
        data
    }

    fn generate_random_buffer(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(rng.gen::<u8>());
        }
        data
    }

    #[test]
    fn pack_unpack_test() {
        let p = Poly { coeffs: [20; N] };
        let mut buffer = zero_initialise_buffer(3 * 128).as_slice();
        buffer.pack(p);

        let mut comp_p = Poly::new();
        comp_p.unpack(buffer);

        assert_eq!(comp_p.coeffs, p.coeffs);
    }

    #[test]
    fn compress_decompress_test() {
        for sec_level in TEST_PARAMS.iter() {
            let buf = generate_random_buffer(sec_level.poly_compressed_bytes()).as_slice();
            let mut buf_comp = zero_initialise_buffer(sec_level.poly_compressed_bytes()).as_slice();

            let mut poly = Poly::new();

            poly.decompress(&buf, sec_level.poly_compressed_bytes());
            buf_comp.compress(poly, sec_level.poly_compressed_bytes());

            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
        }
    }
}
