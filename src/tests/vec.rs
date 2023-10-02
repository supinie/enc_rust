#[cfg(test)]
mod vec_tests {
    use crate::{params::*, poly::*, vec::*, field_ops::*};

    static TEST_PARAMS: [Params; 3] = [
        Params::sec_level_512(),
        Params::sec_level_768(),
        Params::sec_level_1024(),
    ];

    #[test]
    fn add_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec1 = PolyVec::new(&[Poly { coeffs: [20; N] }; 4][0..sec_level.k]).unwrap();
            let mut poly_vec2 = PolyVec::new(&[Poly { coeffs: [20; N] }; 4][0..sec_level.k]).unwrap();
            poly_vec1.add(&poly_vec2);

            assert_eq!(poly_vec1.polys(), &[Poly { coeffs: [20; N] }; 4][0..sec_level.k]);
        }
    }

    #[test]
    fn reduce_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly{ coeffs: [i16::MAX; N] }; 4][0..sec_level.k]).unwrap();
            poly_vec.reduce();
            assert_eq!(poly_vec.polys(), &[Poly { coeffs: [2806; N] }; 4][0..sec_level.k]);
        }
    }

    #[test]
    fn normalise_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly{ coeffs: [i16::MAX; N] }; 4][0..sec_level.k]).unwrap();
            poly_vec.normalise();
            assert_eq!(poly_vec.polys(), &[Poly { coeffs: [cond_sub_q(barrett_reduce(i16::MAX)); N] }; 4][0..sec_level.k]);
        }
    }


    #[test]
    fn ntt_invntt_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut input_vec = PolyVec::new(&[Poly {coeffs: [20; N] }; 4][0..sec_level.k ]).unwrap();
            let mut original_input = input_vec.clone();
            original_input.normalise();
            
            input_vec.ntt();
            input_vec.normalise();
            input_vec.inv_ntt();
            input_vec.normalise();

            for i in 0..sec_level.k {
                for j in 0..N {
                    let p: i32 = input_vec.polys()[i].coeffs[j] as i32;
                    let q: i32 = original_input.polys()[i].coeffs[j] as i32;
                    assert_eq!(
                        p,
                        (q * (1 << 16)) % (Q as i32),
                        "we are testing equality with original in poly {}, index {}",
                        i, j
                    );
                }
            }
        }
    }
}
