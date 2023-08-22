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
        let mut poly_vec1 = PolyVec {
            polynomials: vec![ Poly { coeffs: [20; N] }; TEST_PARAMS[0].k ],
        };
        let mut poly_vec2 = PolyVec {
            polynomials: vec![ Poly { coeffs: [30; N] }; TEST_PARAMS[0].k ],
        };
    }
}
