#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod sample_tests {
    use crate::{
        params::*,
        polynomials::*,
        tests::{params::params_tests::sec_level_strategy, polynomials::poly_tests::*},
    };
    use proptest::prelude::*;

    #[test]
    fn derive_noise_3_reference() {
        let want: [i16; N] = [
            0, 0, 1, -1, 0, 2, 0, -1, -1, 3, 0, 1, -2, -2, 0, 1, -2, 1, 0, -2, 3, 0, 0, 0, 1, 3, 1,
            1, 2, 1, -1, -1, -1, 0, 1, 0, 1, 0, 2, 0, 1, -2, 0, -1, -1, -2, 1, -1, -1, 2, -1, 1, 1,
            2, -3, -1, -1, 0, 0, 0, 0, 1, -1, -2, -2, 0, -2, 0, 0, 0, 1, 0, -1, -1, 1, -2, 2, 0, 0,
            2, -2, 0, 1, 0, 1, 1, 1, 0, 1, -2, -1, -2, -1, 1, 0, 0, 0, 0, 0, 1, 0, -1, -1, 0, -1,
            1, 0, 1, 0, -1, -1, 0, -2, 2, 0, -2, 1, -1, 0, 1, -1, -1, 2, 1, 0, 0, -2, -1, 2, 0, 0,
            0, -1, -1, 3, 1, 0, 1, 0, 1, 0, 2, 1, 0, 0, 1, 0, 1, 0, 0, -1, -1, -1, 0, 1, 3, 1, 0,
            1, 0, 1, -1, -1, -1, -1, 0, 0, -2, -1, -1, 2, 0, 1, 0, 1, 0, 2, -2, 0, 1, 1, -3, -1,
            -2, -1, 0, 1, 0, 1, -2, 2, 2, 1, 1, 0, -1, 0, -1, -1, 1, 0, -1, 2, 1, -1, 1, 2, -2, 1,
            2, 0, 1, 2, 1, 0, 0, 2, 1, 2, 1, 0, 2, 1, 0, 0, -1, -1, 1, -1, 0, 1, -1, 2, 2, 0, 0,
            -1, 1, 1, 1, 1, 0, 0, -2, 0, -1, 1, 2, 0, 0, 1, 1, -1, 1, 0, 1,
        ];

        let seed: [u8; 32] = core::array::from_fn(|i| i as u8);

        let noise = Poly::derive_noise(&seed, 37, Eta::Three);

        assert_eq!(noise.coeffs(), &want);
    }

    #[test]
    fn derive_noise_2_reference() {
        let want: [i16; N] = [
            1, 0, 1, -1, -1, -2, -1, -1, 2, 0, -1, 0, 0, -1, 1, 1, -1, 1, 0, 2, -2, 0, 1, 2, 0, 0,
            -1, 1, 0, -1, 1, -1, 1, 2, 1, 1, 0, -1, 1, -1, -2, -1, 1, -1, -1, -1, 2, -1, -1, 0, 0,
            1, 1, -1, 1, 1, 1, 1, -1, -2, 0, 1, 0, 0, 2, 1, -1, 2, 0, 0, 1, 1, 0, -1, 0, 0, -1, -1,
            2, 0, 1, -1, 2, -1, -1, -1, -1, 0, -2, 0, 2, 1, 0, 0, 0, -1, 0, 0, 0, -1, -1, 0, -1,
            -1, 0, -1, 0, 0, -2, 1, 1, 0, 1, 0, 1, 0, 1, 1, -1, 2, 0, 1, -1, 1, 2, 0, 0, 0, 0, -1,
            -1, -1, 0, 1, 0, -1, 2, 0, 0, 1, 1, 1, 0, 1, -1, 1, 2, 1, 0, 2, -1, 1, -1, -2, -1, -2,
            -1, 1, 0, -2, -2, -1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 2, 2, 0, 1, 0, -1, -1, 0, 2, 0, 0, -2,
            1, 0, 2, 1, -1, -2, 0, 0, -1, 1, 1, 0, 0, 2, 0, 1, 1, -2, 1, -2, 1, 1, 0, 2, 0, -1, 0,
            -1, 0, 1, 2, 0, 1, 0, -2, 1, -2, -2, 1, -1, 0, -1, 1, 1, 0, 0, 0, 1, 0, -1, 1, 1, 0, 0,
            0, 0, 1, 0, 1, -1, 0, 1, -1, -1, 2, 0, 0, 1, -1, 0, 1, -1, 0,
        ];

        let seed: [u8; 32] = core::array::from_fn(|i| i as u8);

        let noise = Poly::derive_noise(&seed, 37, Eta::Two);

        assert_eq!(noise.coeffs(), &want);
    }

    #[test]
    fn derive_uniform_reference() {
        let want: [i16; N] = [
            797, 993, 161, 6, 2608, 2385, 2096, 2661, 1676, 247, 2440, 342, 634, 194, 1570, 2848,
            986, 684, 3148, 3208, 2018, 351, 2288, 612, 1394, 170, 1521, 3119, 58, 596, 2093, 1549,
            409, 2156, 1934, 1730, 1324, 388, 446, 418, 1719, 2202, 1812, 98, 1019, 2369, 214,
            2699, 28, 1523, 2824, 273, 402, 2899, 246, 210, 1288, 863, 2708, 177, 3076, 349, 44,
            949, 854, 1371, 957, 292, 2502, 1617, 1501, 254, 7, 1761, 2581, 2206, 2655, 1211, 629,
            1274, 2358, 816, 2766, 2115, 2985, 1006, 2433, 856, 2596, 3192, 1, 1378, 2345, 707,
            1891, 1669, 536, 1221, 710, 2511, 120, 1176, 322, 1897, 2309, 595, 2950, 1171, 801,
            1848, 695, 2912, 1396, 1931, 1775, 2904, 893, 2507, 1810, 2873, 253, 1529, 1047, 2615,
            1687, 831, 1414, 965, 3169, 1887, 753, 3246, 1937, 115, 2953, 586, 545, 1621, 1667,
            3187, 1654, 1988, 1857, 512, 1239, 1219, 898, 3106, 391, 1331, 2228, 3169, 586, 2412,
            845, 768, 156, 662, 478, 1693, 2632, 573, 2434, 1671, 173, 969, 364, 1663, 2701, 2169,
            813, 1000, 1471, 720, 2431, 2530, 3161, 733, 1691, 527, 2634, 335, 26, 2377, 1707, 767,
            3020, 950, 502, 426, 1138, 3208, 2607, 2389, 44, 1358, 1392, 2334, 875, 2097, 173,
            1697, 2578, 942, 1817, 974, 1165, 2853, 1958, 2973, 3282, 271, 1236, 1677, 2230, 673,
            1554, 96, 242, 1729, 2518, 1884, 2272, 71, 1382, 924, 1807, 1610, 456, 1148, 2479,
            2152, 238, 2208, 2329, 713, 1175, 1196, 757, 1078, 3190, 3169, 708, 3117, 154, 1751,
            3225, 1364, 154, 23, 2842, 1105, 1419, 79, 5, 2013,
        ];

        let seed: [u8; 32] = core::array::from_fn(|i| i as u8);

        let noise = Poly::derive_uniform(&seed, 1, 0).unwrap().normalise();

        assert_eq!(noise.coeffs(), &want);
    }

    proptest! {
        #[test]
        fn derive_noise_test(
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            nonce in (u8::MIN..u8::MAX),
            sec_level in sec_level_strategy(),
        ) {
            let poly_1 = Poly::derive_noise(&seed, nonce, sec_level.eta_1());
            let poly_2 = Poly::derive_noise(&seed, nonce, sec_level.eta_2());
        }

        #[test]
        fn derive_uniform_test(
            seed in prop::array::uniform32(u8::MIN..u8::MAX),
            x in (u8::MIN..u8::MAX),
            y in (u8::MIN..u8::MAX),
        ) {
            let poly = Poly::derive_uniform(&seed, x, y).unwrap();
        }
    }
}
