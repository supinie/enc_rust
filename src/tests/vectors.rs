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
        let poly = Poly{ coeffs: [20; N] };
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec_1 = PolyVec512::from([poly; 2]);
                let poly_vec_2 = PolyVec512::from([poly; 2]);

                poly_vec_1.add(poly_vec_2);

                assert_eq!(poly_vec_1, PolyVec512::from([Poly{ coeffs: [40; N] }; 2]));
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec_1 = PolyVec768::from([poly; 3]);
                let poly_vec_2 = PolyVec768::from([poly; 3]);

                poly_vec_1.add(poly_vec_2);

                assert_eq!(poly_vec_1, PolyVec768::from([Poly{ coeffs: [40; N] }; 3]));
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec_1 = PolyVec1024::from([poly; 4]);
                let poly_vec_2 = PolyVec1024::from([poly; 4]);

                poly_vec_1.add(poly_vec_2);

                assert_eq!(poly_vec_1, PolyVec1024::from([Poly{ coeffs: [40; N] }; 4]));
            }
        }
    }

    #[test]
    fn reduce_test() {
        let poly = Poly{ coeffs: [i16::MAX; N]};
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec_1 = PolyVec512::from([poly; 2]);

                poly_vec_1.reduce();

                assert_eq!(poly_vec_1, PolyVec512::from([Poly{ coeffs: [2806; N] }; 2]));
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec_1 = PolyVec768::from([poly; 3]);

                poly_vec_1.reduce();

                assert_eq!(poly_vec_1, PolyVec768::from([Poly{ coeffs: [2806; N] }; 3]));
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec_1 = PolyVec1024::from([poly; 4]);

                poly_vec_1.reduce();

                assert_eq!(poly_vec_1, PolyVec1024::from([Poly{ coeffs: [2806; N] }; 4]));
            }
        }
    }

    #[test]
    fn normalise_test() {
        let poly = Poly{ coeffs: [i16::MAX; N]};
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec_1 = PolyVec512::from([poly; 2]);

                poly_vec_1.normalise();

                assert_eq!(poly_vec_1, PolyVec512::from([Poly{ coeffs: [conditional_sub_q(barrett_reduce(i16::MAX)); N] }; 2]));
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec_1 = PolyVec768::from([poly; 3]);

                poly_vec_1.normalise();

                assert_eq!(poly_vec_1, PolyVec768::from([Poly{ coeffs: [conditional_sub_q(barrett_reduce(i16::MAX)); N] }; 3]));
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec_1 = PolyVec1024::from([poly; 4]);

                poly_vec_1.normalise();

                assert_eq!(poly_vec_1, PolyVec1024::from([Poly{ coeffs: [conditional_sub_q(barrett_reduce(i16::MAX)); N] }; 4]));
            }
        }
    }


    #[test]
    fn ntt_invntt_test() {
        let poly = Poly{ coeffs: [20; N]};
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec_1 = PolyVec512::from([poly; 2]);
                let mut original_input = poly_vec_1.clone();

                original_input.normalise();

                poly_vec_1.ntt();
                poly_vec_1.normalise();
                poly_vec_1.inv_ntt();
                poly_vec_1.normalise();

                for i in 0..sec_level.k().into() {
                    for j in 0..N {
                        let p: i32 = poly_vec_1[i].coeffs[j] as i32;
                        let q: i32 = original_input[i].coeffs[j] as i32;
                        assert_eq!(
                            p,
                            (q * (1 << 16) % (Q as i32)),
                            "testing equality with original in poly {}, index {}", i, j
                        );
                    }
                }
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec_1 = PolyVec768::from([poly; 3]);
                let mut original_input = poly_vec_1.clone();

                original_input.normalise();

                poly_vec_1.ntt();
                poly_vec_1.normalise();
                poly_vec_1.inv_ntt();
                poly_vec_1.normalise();

                for i in 0..sec_level.k().into() {
                    for j in 0..N {
                        let p: i32 = poly_vec_1[i].coeffs[j] as i32;
                        let q: i32 = original_input[i].coeffs[j] as i32;
                        assert_eq!(
                            p,
                            (q * (1 << 16) % (Q as i32)),
                            "testing equality with original in poly {}, index {}", i, j
                        );
                    }
                }
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec_1 = PolyVec1024::from([poly; 4]);
                let mut original_input = poly_vec_1.clone();

                original_input.normalise();

                poly_vec_1.ntt();
                poly_vec_1.normalise();
                poly_vec_1.inv_ntt();
                poly_vec_1.normalise();

                for i in 0..sec_level.k().into() {
                    for j in 0..N {
                        let p: i32 = poly_vec_1[i].coeffs[j] as i32;
                        let q: i32 = original_input[i].coeffs[j] as i32;
                        assert_eq!(
                            p,
                            (q * (1 << 16) % (Q as i32)),
                            "testing equality with original in poly {}, index {}", i, j
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn derive_noise_range_test() {
        let poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec = PolyVec512::from([poly; 2]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    range_test(&poly, sec_level.eta_1().into());
                }
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec = PolyVec768::from([poly; 3]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    range_test(&poly, sec_level.eta_1().into());
                }
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec = PolyVec1024::from([poly; 4]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    range_test(&poly, sec_level.eta_1().into());
                }
            }
        }
    }


    #[test]
    fn derive_noise_dist_test() {
        let poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();
        for sec_level in TEST_PARAMS.iter() {
            if let &SecurityLevel::FiveOneTwo { .. } = sec_level {
                let mut poly_vec = PolyVec512::from([poly; 2]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    dist_test(&poly, sec_level.eta_1().into());
                }
            }
            if let &SecurityLevel::SevenSixEight { .. } = sec_level {
                let mut poly_vec = PolyVec768::from([poly; 3]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    dist_test(&poly, sec_level.eta_1().into());
                }
            }
            if let &SecurityLevel::TenTwoFour { .. } = sec_level {
                let mut poly_vec = PolyVec1024::from([poly; 4]);

                poly_vec.derive_noise(&seed, nonce, sec_level.eta_1().into());

                for poly in poly_vec.iter() {
                    dist_test(&poly, sec_level.eta_1().into());
                }
            }
        }
    }
}
