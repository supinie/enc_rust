#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod poly_tests {
    use crate::{
        field_operations::*, params::*, polynomials::*,
        tests::params::params_tests::sec_level_strategy,
    };
    use more_asserts::assert_le;
    use proptest::prelude::*;

    const compress_decompress_buf: [u8; 128] = [
        0, 0, 0, 0, 0, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 34, 34, 34, 34, 34, 34, 34, 34,
        34, 34, 50, 51, 51, 51, 51, 51, 51, 51, 51, 51, 67, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68,
        85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 101, 102, 102, 102, 102, 102, 102, 102, 102, 102,
        102, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 135, 136, 136, 136, 136, 136, 136,
        136, 136, 136, 152, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 170, 170, 170, 170,
        170, 170, 170, 170, 170, 170, 186, 187, 187, 187, 187, 187, 187, 187, 187, 187, 187, 204,
        204, 204, 204, 204, 204, 204, 204,
    ];

    const pack_unpack_buf: [u8; 384] = [
        0, 160, 0, 20, 224, 1, 40, 32, 3, 60, 96, 4, 80, 160, 5, 100, 224, 6, 120, 32, 8, 140, 96,
        9, 160, 160, 10, 180, 224, 11, 200, 32, 13, 220, 96, 14, 240, 160, 15, 4, 225, 16, 24, 33,
        18, 44, 97, 19, 64, 161, 20, 84, 225, 21, 104, 33, 23, 124, 97, 24, 144, 161, 25, 164, 225,
        26, 184, 33, 28, 204, 97, 29, 224, 161, 30, 244, 225, 31, 8, 34, 33, 28, 98, 34, 48, 162,
        35, 68, 226, 36, 88, 34, 38, 108, 98, 39, 128, 162, 40, 148, 226, 41, 168, 34, 43, 188, 98,
        44, 208, 162, 45, 228, 226, 46, 248, 34, 48, 12, 99, 49, 32, 163, 50, 52, 227, 51, 72, 35,
        53, 92, 99, 54, 112, 163, 55, 132, 227, 56, 152, 35, 58, 172, 99, 59, 192, 163, 60, 212,
        227, 61, 232, 35, 63, 252, 99, 64, 16, 164, 65, 36, 228, 66, 56, 36, 68, 76, 100, 69, 96,
        164, 70, 116, 228, 71, 136, 36, 73, 156, 100, 74, 176, 164, 75, 196, 228, 76, 216, 36, 78,
        236, 100, 79, 0, 165, 80, 20, 229, 81, 40, 37, 83, 60, 101, 84, 80, 165, 85, 100, 229, 86,
        120, 37, 88, 140, 101, 89, 160, 165, 90, 180, 229, 91, 200, 37, 93, 220, 101, 94, 240, 165,
        95, 4, 230, 96, 24, 38, 98, 44, 102, 99, 64, 166, 100, 84, 230, 101, 104, 38, 103, 124,
        102, 104, 144, 166, 105, 164, 230, 106, 184, 38, 108, 204, 102, 109, 224, 166, 110, 244,
        230, 111, 8, 39, 113, 28, 103, 114, 48, 167, 115, 68, 231, 116, 88, 39, 118, 108, 103, 119,
        128, 167, 120, 148, 231, 121, 168, 39, 123, 188, 103, 124, 208, 167, 125, 228, 231, 126,
        248, 39, 128, 12, 104, 129, 32, 168, 130, 52, 232, 131, 72, 40, 133, 92, 104, 134, 112,
        168, 135, 132, 232, 136, 152, 40, 138, 172, 104, 139, 192, 168, 140, 212, 232, 141, 232,
        40, 143, 252, 104, 144, 16, 169, 145, 36, 233, 146, 56, 41, 148, 76, 105, 149, 96, 169,
        150, 116, 233, 151, 136, 41, 153, 156, 105, 154, 176, 169, 155, 196, 233, 156, 216, 41,
        158, 236, 105, 159,
    ];

    const MSG_BUF: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 3,
    ];

    pub(in crate::tests) fn new_limited_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(-(i16::MAX / 2)..(i16::MAX / 2)) // pick i16::MAX / 2, which should be plenty more
                                                              // than Q whilst ensuring no overflows (we know
                                                              // they can happen)
    }

    pub(in crate::tests) fn new_poly_array() -> impl Strategy<Value = [i16; N]> {
        prop::array::uniform(i16::MIN..i16::MAX)
    }

    prop_compose! {
        pub(in crate::tests) fn new_poly()
            (coeffs in new_poly_array())
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    prop_compose! {
        pub(in crate::tests) fn new_limited_poly()
            (coeffs in new_limited_poly_array())
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    prop_compose! {
        pub(in crate::tests) fn new_ntt_poly()
            (coeffs in prop::array::uniform(-(3713i16)..(3713i16)))
            -> Poly<Unreduced> {
                Poly::from_arr(&coeffs)
            }
    }

    #[test]
    fn compare_compress_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);

        let poly = Poly::from_arr(&coeffs);
        let mut buf = [0u8; 128];
        let _ = poly
            .normalise()
            .compress(&mut buf, &SecurityLevel::new(K::Three))
            .unwrap();

        assert_eq!(buf, compress_decompress_buf);
    }

    #[test]
    fn compare_decompress_test() {
        let poly =
            Poly::decompress(&compress_decompress_buf, &SecurityLevel::new(K::Three)).unwrap();

        let coeffs: [i16; N] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 208,
            208, 208, 208, 208, 208, 208, 208, 208, 208, 208, 416, 416, 416, 416, 416, 416, 416,
            416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 416, 624, 624, 624,
            624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624, 624,
            832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832, 832,
            832, 832, 832, 832, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040,
            1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1040, 1248, 1248, 1248, 1248,
            1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248, 1248,
            1248, 1248, 1248, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456,
            1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1456, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873,
            1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 1873, 2081, 2081, 2081, 2081, 2081,
            2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081, 2081,
            2081, 2081, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289,
            2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2289, 2497, 2497, 2497, 2497, 2497,
            2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497, 2497,
        ];
        let comp_poly = Poly::from_arr(&coeffs).normalise();

        assert_eq!(poly, comp_poly);
    }

    #[test]
    fn compare_pack_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);

        let poly = Poly::from_arr(&coeffs);
        let buf = poly.normalise().pack();

        assert_eq!(buf, pack_unpack_buf);
    }

    #[test]
    fn compare_unpack_test() {
        let poly = Poly::unpack(&pack_unpack_buf).unwrap();

        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);
        let comp_poly = Poly::from_arr(&coeffs);

        assert_eq!(poly, comp_poly);
    }

    #[test]
    fn compare_msg_test() {
        let coeffs: [i16; N] = core::array::from_fn(|i| (i * 10) as i16);
        let poly = Poly::from_arr(&coeffs);
        let buf = poly.normalise().write_msg().unwrap();

        assert_eq!(buf, MSG_BUF);
    }

    #[test]
    fn compare_read_msg_test() {
        let poly = Poly::read_msg(&MSG_BUF).unwrap().normalise();

        let comp_poly = Poly::from_arr(&[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
            1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 1665, 0, 0, 0, 0, 0, 0,
        ])
        .normalise();
        assert_eq!(poly, comp_poly);
    }

    #[test]
    fn compare_barrett_reduce_test() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).barrett_reduce();

        let want: [i16; N] = [
            0, 127, 254, 381, 508, 635, 762, 889, 1016, 1143, 1270, 1397, 1524, 1651, 1778, 1905,
            2032, 2159, 2286, 2413, 2540, 2667, 2794, 2921, 3048, 3175, 3302, 100, 227, 354, 481,
            608, 735, 862, 989, 1116, 1243, 1370, 1497, 1624, 1751, 1878, 2005, 2132, 2259, 2386,
            2513, 2640, 2767, 2894, 3021, 3148, 3275, 73, 200, 327, 454, 581, 708, 835, 962, 1089,
            1216, 1343, 1470, 1597, 1724, 1851, 1978, 2105, 2232, 2359, 2486, 2613, 2740, 2867,
            2994, 3121, 3248, 46, 173, 300, 427, 554, 681, 808, 935, 1062, 1189, 1316, 1443, 1570,
            1697, 1824, 1951, 2078, 2205, 2332, 2459, 2586, 2713, 2840, 2967, 3094, 3221, 19, 146,
            273, 400, 527, 654, 781, 908, 1035, 1162, 1289, 1416, 1543, 1670, 1797, 1924, 2051,
            2178, 2305, 2432, 2559, 2686, 2813, 2940, 3067, 3194, 3321, 119, 246, 373, 500, 627,
            754, 881, 1008, 1135, 1262, 1389, 1516, 1643, 1770, 1897, 2024, 2151, 2278, 2405, 2532,
            2659, 2786, 2913, 3040, 3167, 3294, 92, 219, 346, 473, 600, 727, 854, 981, 1108, 1235,
            1362, 1489, 1616, 1743, 1870, 1997, 2124, 2251, 2378, 2505, 2632, 2759, 2886, 3013,
            3140, 3267, 65, 192, 319, 446, 573, 700, 827, 954, 1081, 1208, 1335, 1462, 1589, 1716,
            1843, 1970, 2097, 2224, 2351, 2478, 2605, 2732, 2859, 2986, 3113, 3240, 38, 165, 292,
            419, 546, 673, 800, 927, 1054, 1181, 1308, 1435, 1562, 1689, 1816, 1943, 2070, 2197,
            2324, 2451, 2578, 2705, 2832, 2959, 3086, 3213, 11, 138, 265, 392, 519, 646, 773, 900,
            1027, 1154, 1281, 1408, 1535, 1662, 1789, 1916, 2043, 2170, 2297, 2424,
        ];
        assert_eq!(poly.coeffs(), &want);
    }

    #[test]
    fn compare_mont_form_test() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).mont_form();

        let want: [i16; N] = [
            0, 572, 1144, -1613, -1041, -469, 103, 675, 1247, -1510, -938, -366, 206, 778, 1350,
            -1407, -835, -263, 309, 881, 1453, -1304, -732, -160, 412, 984, 1556, -1201, -629, -57,
            515, 1087, 1659, -1098, -526, 46, 618, 1190, 1762, -995, -423, 149, 721, 1293, -1464,
            -892, -320, 252, 824, 1396, -1361, -789, -217, 355, 927, 1499, -1258, -686, -114, 458,
            1030, 1602, -1155, -583, -11, 561, 1133, 1705, -1052, -480, 92, 664, 1236, 1808, -949,
            -377, 195, 767, 1339, -1418, -846, -274, 298, 870, 1442, -1315, -743, -171, 401, 973,
            1545, -1212, -640, -68, 504, 1076, 1648, -1109, -537, 35, 607, 1179, 1751, -1006, -434,
            138, 710, 1282, 1854, -903, -331, 241, 813, 1385, 1957, -800, -228, 344, 916, 1488,
            -1269, -697, -125, 447, 1019, 1591, -1166, -594, -22, 550, 1122, 1694, -1063, -491, 81,
            653, 1225, 1797, -960, -388, 184, 756, 1328, 1900, -857, -285, 287, 859, 1431, 2003,
            -754, -182, 390, 962, 1534, -1223, -651, -79, 493, 1065, 1637, -1120, -548, 24, 596,
            1168, 1740, -1017, -445, 127, 699, 1271, 1843, -914, -342, 230, 802, 1374, 1946, -811,
            -239, 333, 905, 1477, 2049, -708, -136, 436, 1008, 1580, 2152, -605, -33, 539, 1111,
            1683, -1074, -502, 70, 642, 1214, 1786, -971, -399, 173, 745, 1317, 1889, -868, -296,
            276, 848, 1420, 1992, -765, -193, 379, 951, 1523, 2095, -662, -90, 482, 1054, 1626,
            2198, -559, 13, 585, 1157, 1729, -1028, -456, 116, 688, 1260, 1832, -925, -353, 219,
            791, 1363, 1935, -822, -250, 322, 894, 1466, 2038, -719, -147, 425, 997, 1569, 2141,
            -616,
        ];
        assert_eq!(poly.coeffs(), &want);
    }

    #[test]
    fn compare_normalise_test() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).normalise();

        let want: [i16; N] = [
            0, 127, 254, 381, 508, 635, 762, 889, 1016, 1143, 1270, 1397, 1524, 1651, 1778, 1905,
            2032, 2159, 2286, 2413, 2540, 2667, 2794, 2921, 3048, 3175, 3302, 100, 227, 354, 481,
            608, 735, 862, 989, 1116, 1243, 1370, 1497, 1624, 1751, 1878, 2005, 2132, 2259, 2386,
            2513, 2640, 2767, 2894, 3021, 3148, 3275, 73, 200, 327, 454, 581, 708, 835, 962, 1089,
            1216, 1343, 1470, 1597, 1724, 1851, 1978, 2105, 2232, 2359, 2486, 2613, 2740, 2867,
            2994, 3121, 3248, 46, 173, 300, 427, 554, 681, 808, 935, 1062, 1189, 1316, 1443, 1570,
            1697, 1824, 1951, 2078, 2205, 2332, 2459, 2586, 2713, 2840, 2967, 3094, 3221, 19, 146,
            273, 400, 527, 654, 781, 908, 1035, 1162, 1289, 1416, 1543, 1670, 1797, 1924, 2051,
            2178, 2305, 2432, 2559, 2686, 2813, 2940, 3067, 3194, 3321, 119, 246, 373, 500, 627,
            754, 881, 1008, 1135, 1262, 1389, 1516, 1643, 1770, 1897, 2024, 2151, 2278, 2405, 2532,
            2659, 2786, 2913, 3040, 3167, 3294, 92, 219, 346, 473, 600, 727, 854, 981, 1108, 1235,
            1362, 1489, 1616, 1743, 1870, 1997, 2124, 2251, 2378, 2505, 2632, 2759, 2886, 3013,
            3140, 3267, 65, 192, 319, 446, 573, 700, 827, 954, 1081, 1208, 1335, 1462, 1589, 1716,
            1843, 1970, 2097, 2224, 2351, 2478, 2605, 2732, 2859, 2986, 3113, 3240, 38, 165, 292,
            419, 546, 673, 800, 927, 1054, 1181, 1308, 1435, 1562, 1689, 1816, 1943, 2070, 2197,
            2324, 2451, 2578, 2705, 2832, 2959, 3086, 3213, 11, 138, 265, 392, 519, 646, 773, 900,
            1027, 1154, 1281, 1408, 1535, 1662, 1789, 1916, 2043, 2170, 2297, 2424,
        ];
        assert_eq!(poly.coeffs(), &want);
    }

    #[test]
    fn compare_pointwise_mul_test() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let p = Poly::from_arr(&coeffs).mont_form();
        let q = Poly::from_arr(&coeffs).mont_form();

        let output = p.pointwise_mul(&q);
        let want = [
            -111, 0, 1952, -470, -403, 2872, -1804, -3290, -1705, 1018, 941, 2480, 2353, 1096,
            -209, -3134, 1363, 3106, -650, -158, -515, 390, -80, -1908, 1725, -394, 1089, -1726,
            -993, 754, 2789, 388, 351, -2824, -802, -2224, 267, 2188, -432, -2904, -576, 2474, 175,
            -1652, -348, -1966, 2774, 1532, 157, 2184, -1071, -10, -2794, 1608, 341, 380, 754,
            2964, -1484, 2702, -2025, -406, 987, 298, -154, -1844, -1592, -174, 1256, -1350, -786,
            1286, -1574, 1076, -588, -1980, -163, -1224, 2853, -3314, -908, -1592, 1561, -2716,
            886, -28, 1203, -186, -350, -3190, 345, -2382, -1962, 2238, 2225, -2646, -3027, 2940,
            1610, -978, -22, -1084, -2545, 2622, 1729, -3176, 1011, 1496, -510, -3336, -244, 2302,
            742, -1564, 503, -1618, -998, 2140, -979, 3052, -2030, 1118, 1511, 2996, 2714, 2028,
            -758, -1786, -2944, -1788, 2640, 2022, -861, 2986, -70, 1104, 462, 3034, -981, 2118,
            -104, -1644, -722, -1594, 1791, 2268, -1196, 3284, -1167, 1454, 332, -3222, -1028,
            2572, 1894, -1138, -55, -1036, -2246, 2878, -768, -2712, -594, 2168, -1814, -2456,
            1039, -3268, -1673, -268, 663, -114, 2013, -2806, 874, -1686, -1174, 3246, -2559,
            -1326, -402, -2086, -1380, 966, -1167, 1172, 531, -1468, 177, -296, -133, -1970, 2182,
            168, -59, -540, -365, 2564, -1329, 2822, -2127, 234, 1171, 1458, 1693, -164, -1362,
            2026, 1451, 1370, 430, -2132, -2406, -1822, -38, 2300, 864, -3082, 2260, 2006, 461,
            -2410, 1569, -3014, -966, 194, 1966, 556, -26, -1928, 414, -600, 149, -2118, -356, 176,
            681, -376, 419, 2884, 193, 3298, 947, 866, 2, 2246, -1572, 780, 260, 3126, -230, 2626,
            943, -720, -728, -254,
        ];
        assert_eq!(output.coeffs(), &want);
    }

    #[test]
    fn new_test() {
        let poly = Poly::new();
    }

    proptest! {
        #[test]
        fn from_arr_test(a in new_poly_array()) {
            let poly = Poly::from_arr(&a);

            assert_eq!(&a, poly.coeffs());
        }

        #[test]
        fn add_test(
            a in new_limited_poly(),
            b in new_limited_poly()
        ) {
            let outout = a.add(&b);
        }

        #[test]
        fn sub_test(
            a in new_limited_poly(),
            b in new_limited_poly()
        ) {
            let outout = a.sub(&b);
        }

        #[test]
        fn barrett_reduce_test(poly in new_poly()) {
            let output = poly.barrett_reduce();
        }

        #[test]
        fn mont_form_test(poly in new_poly()) {
            let output = poly.mont_form();
        }

        #[test]
        fn normalise_test(poly in new_poly()) {
            let output = poly.normalise();
        }

        #[test]
        fn pointwise_mul_test(
            a in new_poly(),
            b in new_poly()
        ) {
            let outout = a.normalise().pointwise_mul(&b.normalise());
        }

        #[test]
        fn pointwise_mul_test_alt(
            a in new_poly(),
            b in new_poly()
        ) {
            let ah = a.normalise().ntt();
            let bh = b.normalise().ntt();
            let ph = ah.pointwise_mul(&bh)
                .barrett_reduce()
                .inv_ntt()
                .normalise();

            let a_coeffs = a.coeffs();
            let b_coeffs = b.coeffs();
            let mut p_coeffs = [0i16; N];

            for i in 0..N {
                for j in 0..N {
                    let mut v = montgomery_reduce((a_coeffs[i] as i32) * (b_coeffs[j] as i32));
                    let mut k = i + j;
                    if k >= N {
                        k -= N;
                        v = -v;
                    }
                    p_coeffs[k] = barrett_reduce(v + p_coeffs[k]);
                }
            }

            for i in 0..N {
                p_coeffs[i] = ((p_coeffs[i] as i32) * ((1 << 16) % (Q as i32)) % (Q as i32)) as i16;
            }

            let p = Poly::from_arr(&p_coeffs).normalise();

            assert_eq!(p, ph);
        }

        #[test]
        fn pack_test(poly in new_poly()) {
            let output = poly.normalise().pack();
        }

        #[test]
        fn write_msg_test(poly in new_poly()) {
            let msg = poly.normalise().write_msg().unwrap();
        }

        #[test]
        fn compress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let result = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level)
                .unwrap();
        }

        #[test]
        fn unpack_test(poly in new_poly()) {
            let buf = poly.normalise().pack();

            let unpacked = Poly::unpack(&buf).unwrap();
        }

        #[test]
        fn read_msg_test(msg in prop::array::uniform32(0u8..)) {
            let poly = Poly::read_msg(&msg).unwrap();
        }

        #[test]
        fn decompress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160]; // max poly_compressed_bytes
            let _ = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level);

            let decompressed_poly = Poly::decompress(&buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();
        }

        #[test]
        fn pack_unpack_test(poly in new_poly()) {
            let buf = poly.normalise().pack();

            let unpacked = Poly::unpack(&buf).unwrap();

            assert_eq!(poly.normalise(), unpacked.normalise());
        }

        #[test]
        fn compress_decompress_test(
            poly in new_poly(),
            sec_level in sec_level_strategy()
        ) {
            let mut buf = [0u8; 160];
            let _ = poly
                .normalise()
                .compress(&mut buf[..sec_level.poly_compressed_bytes()], &sec_level)
                .unwrap();

            let decompressed = Poly::decompress(&buf[..sec_level.poly_compressed_bytes()], &sec_level).unwrap();

            for (original_coeff, new_coeff) in poly
                .normalise()
                .coeffs()
                .iter()
                .zip(decompressed.coeffs().iter()) {
                    if (original_coeff - new_coeff).abs() < 150 {
                        assert_le!((original_coeff - new_coeff).abs(), 150, "original: {original_coeff}, new: {new_coeff}");
                    } else {
                        assert_le!(Q as i16 - (original_coeff - new_coeff).abs(), 150, "original: {original_coeff}, new: {new_coeff}");
                    }
            }
        }

        #[test]
        fn write_read_msg_test(
            message in prop::array::uniform32(u8::MIN..u8::MAX),
            poly in new_poly()
        ) {
            let poly_from_msg = Poly::read_msg(&message).unwrap();
            let comp_message = poly_from_msg.normalise().write_msg().unwrap();

            assert_eq!(message, comp_message);
        }
    }
}
