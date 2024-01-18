#![allow(warnings)]
#[cfg(test)]
pub(in crate::tests) mod buffer_tests {
    use crate::{params::*, polynomials::*, vectors::*};
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
    fn pack_unpack_poly_test() {
        let poly = Poly { coeffs: [20; N] };
        let mut buffer = [0; POLYBYTES];
        &poly.pack(&mut buffer);

        let mut comp_poly = Poly::new();
        comp_poly.unpack(&buffer);

        assert_eq!(comp_poly.coeffs, poly.coeffs);
    }

    #[test]
    fn compress_decompress_poly_test() {
        for sec_level in TEST_PARAMS.iter() {
            let buf = generate_random_buffer(sec_level.poly_compressed_bytes());
            let mut buf_comp = zero_initialise_buffer(sec_level.poly_compressed_bytes());

            let mut poly = Poly::new();

            poly.decompress(&buf, sec_level);
            poly.compress(&mut buf_comp, sec_level);

            assert_eq!(buf_comp, buf);
        }
    }

    #[test]
    fn pack_unpack_vec_test() {
        let poly = Poly { coeffs: [20; N] };
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut buffer = [0; 2 * POLYBYTES];
                let poly_vec = PolyVec512::from([poly; 2]);
                &poly_vec.pack(&mut buffer);

                let mut comp_poly_vec = PolyVec512::from([Poly::new(); 2]);
                comp_poly_vec.unpack(&buffer);

                assert_eq!(comp_poly_vec, poly_vec);
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut buffer = [0; 3 * POLYBYTES];
                let poly_vec = PolyVec768::from([poly; 3]);
                &poly_vec.pack(&mut buffer);

                let mut comp_poly_vec = PolyVec768::from([Poly::new(); 3]);
                comp_poly_vec.unpack(&buffer);

                assert_eq!(comp_poly_vec, poly_vec);
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut buffer = [0; 4 * POLYBYTES];
                let poly_vec = PolyVec1024::from([poly; 4]);
                &poly_vec.pack(&mut buffer);

                let mut comp_poly_vec = PolyVec1024::from([Poly::new(); 4]);
                comp_poly_vec.unpack(&buffer);

                assert_eq!(comp_poly_vec, poly_vec);
            }
        }
    }

    #[test]
    fn compress_decompress_vec_test() {
        for sec_level in TEST_PARAMS.iter() {
            let k_value: usize = sec_level.k().into();
            let buf = generate_random_buffer(k_value * sec_level.poly_compressed_bytes());
            let mut buf_comp = zero_initialise_buffer(k_value * sec_level.poly_compressed_bytes());

            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec = PolyVec512::from([Poly::new(); 2]);

                poly_vec.decompress(&buf);
                poly_vec.compress(&mut buf_comp);

                assert_eq!(buf_comp, buf);
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec = PolyVec768::from([Poly::new(); 3]);

                poly_vec.decompress(&buf);
                poly_vec.compress(&mut buf_comp);

                assert_eq!(buf_comp, buf);
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec = PolyVec1024::from([Poly::new(); 4]);

                poly_vec.decompress(&buf);
                poly_vec.compress(&mut buf_comp);

                assert_eq!(buf_comp, buf);
            }
        }
    }
}
