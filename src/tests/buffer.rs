#[cfg(test)]
pub(in crate::tests) mod buffer_tests {
    use crate::{params::*, poly::*};
    use rand::Rng;
    extern crate std;
    use std::vec::Vec;
 
    static TEST_PARAMS: [Params; 3] = [
        Params::sec_level_512(),
        Params::sec_level_768(),
        Params::sec_level_1024(),
    ];

    pub(in crate::tests) fn zero_initialise_buffer(size: Option<usize>) -> Vec<u8> {
        let mut data = Vec::with_capacity(size.unwrap());
        for _ in 0..size.unwrap() {
            data.push(0u8);
        }
        data
    }

    fn generate_random_buffer(size: Option<usize>) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(size.unwrap());

        for _ in 0..size.unwrap() {
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

            poly.decompress(&buf, sec_level.poly_compressed_bytes());
            poly.compress(&mut buf_comp, sec_level.poly_compressed_bytes());

            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
            assert_eq!(buf_comp, buf);
        }
    }

    #[test]
    fn decompress_fail_test() {
        for sec_level in TEST_PARAMS.iter() {
            let buf = generate_random_buffer(sec_level.poly_compressed_bytes());
            let mut buf_comp = zero_initialise_buffer(sec_level.poly_compressed_bytes());

            let mut poly = Poly::new();
            // We only need test the match statements, as it is not possible for the try_from to
            // return an error.

            let err = poly.decompress(&buf, Some(120));
            assert_eq!(err.unwrap_err(), DecompressError::InvalidCompressedBytes);

            let err = poly.decompress(&buf, None);
            assert_eq!(err.unwrap_err(), DecompressError::InvalidCompressedBytes);
        }
    }

        #[test]
    fn compress_fail_test() {
        for sec_level in TEST_PARAMS.iter() {
            let buf = generate_random_buffer(sec_level.poly_compressed_bytes());
            let mut buf_comp = zero_initialise_buffer(sec_level.poly_compressed_bytes());

            let mut poly = Poly::new();
            // We only need test the match statements, as it is not possible for the try_from to
            // return an error.

            poly.decompress(&buf, sec_level.poly_compressed_bytes());

            let err = poly.compress(&mut buf_comp, Some(120));
            assert_eq!(err.unwrap_err(), DecompressError::InvalidCompressedBytes);

            let err = poly.decompress(&buf, None);
            assert_eq!(err.unwrap_err(), DecompressError::InvalidCompressedBytes);
        }
    }

}
