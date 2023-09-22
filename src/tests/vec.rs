#[cfg(test)]
mod vec_tests {
    use crate::{params::*, poly::*, vec::*};

    static TEST_PARAMS: [Params; 3] = [
        Params::sec_level_512(),
        Params::sec_level_768(),
        Params::sec_level_1024(),
    ];

    #[test]
    fn add_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec1 = PolyVec {
                polynomials: vec![ Poly { coeffs: [20; N] }; sec_level.k ],
            };
            let mut poly_vec2 = PolyVec {
                polynomials: vec![ Poly { coeffs: [30; N] }; sec_level.k ],
            };

            poly_vec1.add(&poly_vec2);

            assert_eq!(poly_vec1.polynomials, vec![ Poly { coeffs: [50; N] }; sec_level.k ]);
        }
    }

    #[test]
    fn reduce_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec {
                polynomials: vec![ Poly { coeffs: [i16::MAX; N] }; sec_level.k ],
            };
            poly_vec.reduce();
            assert_eq!(poly_vec.polynomials, vec![ Poly { coeffs: [2806; N] }; sec_level.k]);
        }
    }
}
