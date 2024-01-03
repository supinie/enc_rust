#![allow(warnings)]
#[cfg(test)]
pub(in crate::tests) mod buffer_tests {
    use crate::{params::*, polynomials::*};
    use rand::Rng;
    extern crate std;
    use std::vec::Vec;
 
    static TEST_PARAMS: [SecurityLevel; 3] = [
        SecurityLevel::new(K::Two),
        SecurityLevel::new(K::Three),
        SecurityLevel::new(K::Four),
    ];

    pub(in crate::tests) fn zero_initialise_buffer(size: usize) -> Vec<u8> {
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
        let mut buffer = [0; 3 * 128];
        &p.pack(&mut buffer);

        let mut comp_p = Poly::new();
        comp_p.unpack(&buffer);

        assert_eq!(comp_p.coeffs, p.coeffs);
    }

    #[test]
    fn compress_decompress_test() {
        for sec_level in TEST_PARAMS.iter() {
            let buf = generate_random_buffer(sec_level.poly_compressed_bytes());
            let mut buf_comp = zero_initialise_buffer(sec_level.poly_compressed_bytes());

            let mut poly = Poly::new();

            poly.decompress(&buf, sec_level);
            poly.compress(&mut buf_comp, sec_level);

            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
        }
    }
}
