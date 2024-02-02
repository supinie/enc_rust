#![allow(warnings)]
#[cfg(test)]
mod poly_tests {
    use crate::{
        field_operations::{barrett_reduce, montgomery_reduce},
        params::*,
        polynomials::*,
        tests::buffer::buffer_tests::zero_initialise_buffer,    
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn pointwise_mul_test(
            a in prop::array::uniform(-(Q as i16)..(Q as i16)),
            b in prop::array::uniform(-(Q as i16)..(Q as i16))
        ) {
            let mut poly_a = Poly { coeffs: a };
            let poly_b = Poly { coeffs: b };

            let mut a_copy = poly_a;
            let b_copy = poly_b;

            poly_a.pointwise_mul(&poly_b);
            a_copy.pointwise_mul_alt(&poly_b);

            assert_eq!(poly_a, a_copy);

            // a_copy.ntt();
            // a_copy.normalise();
            // b_copy.ntt();
            // b_copy.normalise();

            // a_copy.pointwise_mul(&b_copy);
            // a_copy.barrett_reduce();
            // a_copy.inv_ntt();

            // for i in 0..N {
            //     for j in 0..N {
            //         let mut v = montgomery_reduce((poly_a.coeffs[i] as i32) * (poly_b.coeffs[j] as i32));
            //         let mut k = i + j;

            //         // circular shifting case; x^N = -1
            //         if k >= N {
            //             k -= N;
            //             v = -v;
            //         }
            //         p.coeffs[k] = barrett_reduce(v + p.coeffs[k]);
            //     }
            // }

            // for i in 0..N {
            //     p.coeffs[i] = (((p.coeffs[i] as i32) * ((1 << 16) % (Q as i32))) % (Q as i32)) as i16;
            // }

            // p.normalise();
            // a_copy.normalise();

            // assert_eq!(p.coeffs, a_copy.coeffs);
        }
    }

    const INPUT_COEFFS: [i16; N] = [
        -650, -1557, -1607, 924, 1571, 776, -531, -1418, -1172, -511, 1430, 1180, 892, 1471, 1063,
        934, 1320, -1278, 1420, 687, 834, -1508, 80, 5, -266, -1306, 826, -958, 1079, -705, -1507,
        -1236, -597, -1449, 1405, -638, 1045, -791, -339, 1590, 415, -573, 1105, -305, -715, 555,
        -1036, -1059, 995, -1281, 1293, 216, -1072, -292, -1443, 327, 119, 1100, 1087, -467, -1269,
        1245, 15, -149, 1514, -245, 930, 946, 682, -1073, 923, -516, 19, 364, 844, 969, -694, 1473,
        1627, -1364, -1420, 1255, 570, -827, -650, 792, 1218, 1186, 227, 640, -893, 675, 272, 839,
        -1138, 173, -1071, 1457, -546, 1328, -1281, -1287, -852, 455, 794, -1621, -188, -1565,
        1570, -226, -1211, -1515, -583, 1024, -625, 484, 203, -1072, 6, -1235, 1285, 830, 1106,
        -107, 4, -1645, -1599, 650, 1529, -427, 314, -1416, -49, 1179, 756, -868, 1275, -1045,
        -768, 1180, 395, -262, 1330, 1529, -903, -907, 348, 965, -262, -1259, 1448, -641, 1236,
        889, 917, -373, 961, 1035, -1387, 825, -1057, 644, 1126, 611, 210, 218, 1408, -180, 838,
        -972, -664, -380, 431, -947, -516, 1246, -137, 1550, 546, 1267, -1374, 329, -1039, 1528,
        -395, 1595, -458, -1099, 1017, -128, 1444, -1652, -1149, 905, 624, 778, -542, 420, -1066,
        -1316, 1113, -13, 21, -69, 757, 1223, -488, -1044, 1108, -1554, -1442, 1399, 492, -816,
        1314, -1567, -834, -756, -949, -1482, 781, -1170, -1469, 1350, 1401, 873, 463, -754, -372,
        1114, -353, -872, -564, 1386, 724, -1471, 944, -1376, -850, 439, -1265, -627, 173, 892,
        274, -125, 1042, 1105, 784, -1571, 1340, -48, -1024, 1537, -363, -1236,
    ];

    const OUTPUT_COEFFS: [i16; N] = [
        0, 1665, 1665, 1665, 1665, 0, 0, 1665, 1665, 0, 1665, 1665, 1665, 1665, 1665, 1665, 1665,
        1665, 1665, 0, 1665, 1665, 0, 0, 0, 1665, 0, 1665, 1665, 0, 1665, 1665, 0, 1665, 1665, 0,
        1665, 0, 0, 1665, 0, 0, 1665, 0, 0, 0, 1665, 1665, 1665, 1665, 1665, 0, 1665, 0, 1665, 0,
        0, 1665, 1665, 0, 1665, 1665, 0, 0, 1665, 0, 1665, 1665, 0, 1665, 1665, 0, 0, 0, 1665,
        1665, 0, 1665, 1665, 1665, 1665, 1665, 0, 0, 0, 0, 1665, 1665, 0, 0, 1665, 0, 0, 1665,
        1665, 0, 1665, 1665, 0, 1665, 1665, 1665, 1665, 0, 0, 1665, 0, 1665, 1665, 0, 1665, 1665,
        0, 1665, 0, 0, 0, 1665, 0, 1665, 1665, 0, 1665, 0, 0, 1665, 1665, 0, 1665, 0, 0, 1665, 0,
        1665, 0, 1665, 1665, 1665, 0, 1665, 0, 0, 1665, 1665, 1665, 1665, 0, 1665, 0, 1665, 1665,
        0, 1665, 1665, 1665, 0, 1665, 1665, 1665, 0, 1665, 0, 1665, 0, 0, 0, 1665, 0, 1665, 1665,
        0, 0, 0, 1665, 0, 1665, 0, 1665, 0, 1665, 1665, 0, 1665, 1665, 0, 1665, 0, 1665, 1665, 0,
        1665, 1665, 1665, 1665, 0, 0, 0, 0, 1665, 1665, 1665, 0, 0, 0, 0, 1665, 0, 1665, 1665,
        1665, 1665, 1665, 0, 0, 1665, 1665, 1665, 0, 1665, 1665, 0, 1665, 1665, 1665, 1665, 1665,
        0, 0, 0, 1665, 0, 1665, 0, 1665, 0, 1665, 1665, 1665, 1665, 0, 1665, 0, 0, 1665, 0, 0,
        1665, 1665, 0, 1665, 1665, 0, 1665, 1665, 0, 1665,
    ];

    // Test Poly::new()
    #[test]
    fn new_test() {
        let poly = Poly::new();
        assert_eq!(poly.coeffs, [0; N]);
    }

    // Test Poly::add()
    #[test]
    fn add_test() {
        let mut poly1 = Poly { coeffs: [1; N] };
        let poly2 = Poly { coeffs: [4; N] };
        poly1.add(&poly2);
        assert_eq!(poly1.coeffs, [5; N]);
    }

    // Test Poly::sub()
    #[test]
    fn sub_test() {
        let mut poly1 = Poly { coeffs: [3; N] };
        let poly2 = Poly { coeffs: [1; N] };
        poly1.sub(&poly2);
        assert_eq!(poly1.coeffs, [2; N]);
    }

    #[test]
    fn mont_form_test() {
        let mut poly1 = Poly {
            coeffs: [i16::MAX; N],
        };
        let mut poly2 = Poly {
            coeffs: [i16::MIN; N],
        };

        poly1.mont_form();
        poly2.mont_form();

        assert_eq!(poly1.coeffs, [56; N]);
        assert_eq!(poly2.coeffs, [988; N]);
    }

    // Test Poly::pointwise_mul()
    // #[test]
    // fn pointwise_mul_test() {
    //     let a = Poly {
    //         coeffs: [Q as i16; N],
    //     };
    //     let mut b = Poly { coeffs: [20; N] };
    //     let mut p = Poly::new();

    //     b.coeffs[0] = 1;

    //     let mut a_copy = a;
    //     let mut b_copy = b;

    //     a_copy.ntt();
    //     b_copy.ntt();

    //     a_copy.pointwise_mul(&b_copy);
    //     a_copy.barrett_reduce();
    //     a_copy.inv_ntt();

    //     for i in 0..N {
    //         for j in 0..N {
    //             let mut v = montgomery_reduce((a.coeffs[i] as i32) * (b.coeffs[j] as i32));
    //             let mut k = i + j;

    //             // circular shifting case; x^N = -1
    //             if k >= N {
    //                 k -= N;
    //                 v = -v;
    //             }
    //             p.coeffs[k] = barrett_reduce(v + p.coeffs[k]);
    //         }
    //     }

    //     for i in 0..N {
    //         p.coeffs[i] = (((p.coeffs[i] as i32) * ((1 << 16) % (Q as i32))) % (Q as i32)) as i16;
    //     }

    //     p.normalise();
    //     a_copy.normalise();

    //     assert_eq!(p.coeffs, a_copy.coeffs);
    // }

    #[test]
    fn to_and_from_msg_test() {
        let mut poly_original = Poly {
            coeffs: INPUT_COEFFS,
        };

        let mut msg = zero_initialise_buffer(32);
        poly_original.write_msg(&mut msg);
        poly_original.read_msg(&msg);

        assert_eq!(poly_original.coeffs, OUTPUT_COEFFS);
    }
}
