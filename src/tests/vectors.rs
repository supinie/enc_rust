#![allow(warnings)]
#[cfg(test)]
mod vec_tests {
    use crate::{params::*, polynomials::*, vectors::*, field_operations::*};
    use crate::tests::sample::sample_tests::*;

    static TEST_PARAMS: [SecurityLevel; 3] = [
        SecurityLevel::new(K::Two),
        SecurityLevel::new(K::Three),
        SecurityLevel::new(K::Four),
    ];

    #[test]
    fn add_test() {
    }

    #[test]
    fn reduce_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly{ coeffs: [i16::MAX; N] }; 4][0..sec_level.k_value()]).unwrap();
            poly_vec.reduce();
            assert_eq!(poly_vec.polys(), &[Poly { coeffs: [2806; N] }; 4][0..sec_level.k_value()]);
        }
    }

    #[test]
    fn normalise_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly{ coeffs: [i16::MAX; N] }; 4][0..sec_level.k_value()]).unwrap();
            poly_vec.normalise();
            assert_eq!(poly_vec.polys(), &[Poly { coeffs: [conditional_sub_q(barrett_reduce(i16::MAX)); N] }; 4][0..sec_level.k_value()]);
        }
    }


    #[test]
    fn ntt_invntt_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut input_vec = PolyVec::new(&[Poly {coeffs: [20; N] }; 4][0..sec_level.k_value() ]).unwrap();
            let mut original_input = input_vec.clone();
            original_input.normalise();
            
            input_vec.ntt();
            input_vec.normalise();
            input_vec.inv_ntt();
            input_vec.normalise();

            for i in 0..sec_level.k_value() {
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

    #[test]
    fn derive_noise_range_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly::new(); 4][0..sec_level.k_value() ]).unwrap();
            let seed = generate_random_seed();
            let nonce = generate_random_nonce();

            poly_vec.derive_noise(&seed, nonce, sec_level.eta_1_value());
            for poly in poly_vec.polys().iter() {
                range_test(&poly, sec_level.eta_1_value() as i16);
            }
        }
    }


    #[test]
    fn derive_noise_dist_test() {
        for sec_level in TEST_PARAMS.iter() {
            let mut poly_vec = PolyVec::new(&[Poly::new(); 4][0..sec_level.k_value() ]).unwrap();
            let seed = generate_random_seed();
            let nonce = generate_random_nonce();

            poly_vec.derive_noise(&seed, nonce, sec_level.eta_1_value());
            for poly in poly_vec.polys().iter() {
                dist_test(&poly, sec_level.eta_1_value());
            }
        }
    }


    #[test]
    #[should_panic]
    fn inner_product_pointwise_mismatch_lengths_test() {
        let mut result = Poly::new();
        let poly1 = PolyVec::PolyVec512([Poly::new(); 2]);
        let poly2 = PolyVec::PolyVec768([Poly::new(); 3]);

        result.inner_product_pointwise(&poly1, &poly2);
    }
}
