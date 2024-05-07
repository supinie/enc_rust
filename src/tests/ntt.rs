#![allow(warnings)]
#[cfg(test)]

pub(in crate::tests) mod ntt_tests {
    use crate::{params::*, polynomials::*, tests::polynomials::poly_tests::*};
    use more_asserts::{assert_ge, assert_le, assert_lt};
    use proptest::prelude::*;

    #[rustfmt::skip]
    const INV_NTT_REDUCTIONS: [&[usize]; 7] = [
        &[],
        &[],
        &[16, 17, 48, 49, 80, 81, 112, 113, 144, 145, 176, 177, 208, 209, 240, 241],
        &[0, 1, 32, 33, 34, 35, 64, 65, 96, 97, 98, 99, 128, 129, 160, 161, 162, 163, 192, 193, 224, 225, 226, 227],
        &[2, 3, 66, 67, 68, 69, 70, 71, 130, 131, 194, 195, 196, 197, 198, 199],
        &[4, 5, 6, 7, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143],
        &[]
    ];

    #[test]
    fn compare_ntt_test() {
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).mont_form().ntt();

        let want = [-5463, -541, -6575, -1333, -5158, -25, -2604, 1087, -4536, -1678, -5930, 1414, -2910, 2772, -1088, 3264, -1119, -3415, -1927, -4357, 1526, -5567, 2364, -2913, -4734, -3193, -2942, 53, -1811, 793, -1493, -1625, -3927, 5435, -2549, 2561, -3311, 538, -1049, 3218, 422, 1940, 340, 174, -1693, 2779, 1751, 4715, -1888, -408, -3290, 986, -557, 3221, 1991, 1581, -764, -84, -3096, 3218, 2589, 25, 327, -683, -876, 70, -3114, 1466, -1191, -2834, -667, -1158, -1274, 1491, -2898, 293, -901, -3188, -1719, -1060, -1901, -3824, -4203, -1666, -4082, -2732, -6226, -5610, -1948, -225, -758, 2253, -5581, -2711, -2581, -1461, 2095, -568, 1565, 2572, 5, -2932, 1739, -1800, -2990, -5259, -2330, -1797, -1161, -870, 805, -202, 219, -2202, 3289, -238, -1166, -3002, -1166, -4822, 1109, -4294, 1711, -1688, 2203, -2501, 2281, -61, 2569, -603, 1285, 475, -387, -2502, -631, -742, 2606, 748, 2746, -932, 3998, 2011, 4086, 993, -1109, 1042, -2463, -1610, 996, 1607, -1416, 1997, -1138, 216, -3850, 3560, 62, 3528, 1334, 1812, 278, 3631, 2022, 3625, 710, 5501, 3006, 2427, -845, 3012, -181, 2942, 2094, 4030, 980, 7376, -4017, -203, -1377, -1275, -904, 3707, -3410, 1627, -1517, 118, -81, 2706, -3830, 2721, -3408, 119, -819, 1335, -213, 3495, 1699, -142, 1549, -1316, 3708, -1396, 970, -1664, 580, 777, -1306, 1687, 5264, -3214, 1936, -1392, 2844, 825, 4592, 889, 5125, 2002, 3343, 638, 873, 3565, 2895, 1335, 5623, 3570, 3979, 5848, 5951, 4100, 3535, 3138, 4987, 4750, 2985, 4564, 7573, 4489, 7367, 4149, 3300, 92, 556, 2676, 4834, 1963, 2810, 4425, 1221, 2003, 2363, 1703, -326, -1095, 2810, 2361];
        assert_eq!(poly.coeffs(), &want);
    }

    fn compare_inv_ntt_test() { 
        let coeffs = core::array::from_fn(|i| (i * 127) as i16);
        let poly = Poly::from_arr(&coeffs).mont_form().inv_ntt();

        let want = [942, -335, -719, -719, 612, 612, -917, -917, 768, 768, 303, 303, -484, -484, -17, -17, 1535, 1535, -142, -142, 716, 716, 499, 499, -664, -664, -1484, -1484, -947, -947, -677, -677, -1508, -1508, -1301, -1301, 349, 349, -1224, -1224, 576, 576, 174, 174, -485, -485, -232, -232, -1353, -1353, -82, -82, -1227, -1227, -315, -315, 331, 331, 342, 342, -379, -379, -1485, -1485, -1499, -1499, 178, 178, 977, 977, -280, -280, -810, -810, 1602, 1602, 826, 826, 380, 380, 826, 826, -3, -3, 416, 416, -369, -369, 708, 708, 175, 175, -1457, -1457, -1001, -1001, -1258, -1258, 1248, 1248, -1503, -1503, -1373, -1373, -1114, -1114, -314, -314, -541, -541, -960, -960, -387, -387, -1694, -1694, -884, -884, -591, -591, 749, 749, -716, -716, -1076, -1076, -734, -734, -1649, -1649, -734, -734, -1076, -1076, -716, -716, 749, 749, -591, -591, -884, -884, 1635, 1635, -387, -387, -960, -960, -541, -541, -314, -314, -1114, -1114, -1373, -1373, -1503, -1503, 1248, 1248, -1258, -1258, -1001, -1001, -1457, -1457, 175, 175, 708, 708, -369, -369, 416, 416, -3, -3, 826, 826, 380, 380, 826, 826, 1602, 1602, -810, -810, -280, -280, 977, 977, 178, 178, -1499, -1499, -1485, -1485, -379, -379, 342, 342, 331, 331, -315, -315, -1227, -1227, -82, -82, -1353, -1353, -232, -232, -485, -485, 174, 174, 576, 576, -1224, -1224, 349, 349, -1301, -1301, -1508, -1508, -677, -677, -947, -947, -1484, -1484, -664, -664, 499, 499, 716, 716, -142, -142, 1535, 1535, -17, -17, -484, -484, 303, 303, 768, 768, -917, -917, 612, 612, -719, -719];
        assert_eq!(poly.coeffs(), &want);
    }

    proptest! {
        #[test]
        fn ntt_tests(poly in new_poly()) {
            let output_1 = poly.normalise().ntt();
            let output_2 = poly.mont_form().ntt();
            let output_3 = poly.barrett_reduce().ntt();
        }

        #[test]
        fn ntt_test_alt(poly in new_ntt_poly()) {
            let comp_poly = poly.normalise();

            poly.normalise()
                .ntt()
                .coeffs()
                .iter()
                .for_each(|&coeff| {
                    assert_le!(coeff, (7 * Q) as i16);
                    assert_ge!(coeff, -((7 * Q) as i16));
                });

            poly.normalise()
                .ntt()
                .barrett_reduce()
                .normalise()
                .inv_ntt()
                .coeffs()
                .iter()
                .for_each(|&coeff| {
                    assert_le!(coeff, Q as i16);
                    assert_ge!(coeff, -(Q as i16));
                });

            poly.normalise()
                .ntt()
                .barrett_reduce()
                .normalise()
                .inv_ntt()
                .barrett_reduce()
                .normalise()
                .coeffs()
                .iter()
                .zip(comp_poly.coeffs().iter())
                .for_each(|(&coeff, &comp_coeff)| {
                    assert_eq!(coeff as i32, ((comp_coeff as i32) * (1 << 16)) % (Q as i32));
                });
        }


        #[test]
        fn inv_ntt_test(poly in new_ntt_poly()) {
            let output = poly.inv_ntt();
        }

        #[test]
        fn inv_ntt_test_alt(poly in new_ntt_poly()) {
            let mut xs = [1i16; 256];

            let mut r = -1;

            for (layer, reductions) in (1..8).zip(INV_NTT_REDUCTIONS) {
                let w = 1 << layer;
                let mut i = 0;

                if i + w < 256 {
                    xs[i] = xs[i] + xs[i + w];
                    assert_lt!(xs[i], 9);

                    xs[i + w] = 1;
                    i += 1;

                    if i % w == 0 {
                        i += w;
                    }
                }

                for &i in reductions {
                    xs[i] = 1;
                }
            }
        }
    }
}
