use crate::{
    field_operations::{barrett_reduce, montgomery_reduce},
    params::N,
    polynomials::{Poly, Reduced, State},
};

// precomputed powers of the primative root of unity in Montgomery representation for use in ntt()
#[rustfmt::skip]
pub const ZETAS: [i16; 128] = [
    2285, 2571, 2970, 1812, 1493, 1422, 287, 202, 3158, 622, 1577, 182,
	962, 2127, 1855, 1468, 573, 2004, 264, 383, 2500, 1458, 1727, 3199,
	2648, 1017, 732, 608, 1787, 411, 3124, 1758, 1223, 652, 2777, 1015,
	2036, 1491, 3047, 1785, 516, 3321, 3009, 2663, 1711, 2167, 126,
	1469, 2476, 3239, 3058, 830, 107, 1908, 3082, 2378, 2931, 961, 1821,
	2604, 448, 2264, 677, 2054, 2226, 430, 555, 843, 2078, 871, 1550,
	105, 422, 587, 177, 3094, 3038, 2869, 1574, 1653, 3083, 778, 1159,
	3182, 2552, 1483, 2727, 1119, 1739, 644, 2457, 349, 418, 329, 3173,
	3254, 817, 1097, 603, 610, 1322, 2044, 1864, 384, 2114, 3193, 1218,
	1994, 2455, 220, 2142, 1670, 2144, 1799, 2051, 794, 1819, 2475,
	2459, 478, 3221, 3021, 996, 991, 958, 1869, 1522, 1628,
];

// These keep track of which coeffs to apply Barrett reduction to during inv_ntt()
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

impl<S: State + Reduced + Copy> Poly<S> {
    // Cooley-Tukey radix-2 Decimation in Time (DIT) NTT algorithm
    // coefficients must be bounded in absolute value by q,
    // and the outputs are bounded in absolute value by 7q.
    // If the input is in montgomery or regular form, then so is the output.
    // Example:
    // ```
    // let output_poly = poly.ntt();
    // ```
    pub(crate) fn ntt(&self) -> Self {
        let mut coeffs = self.coeffs;
        let mut k = 0usize;

        // want to start from N / 2 so start from 1 not 0
        for l in (1..).map(|x| N >> x).take_while(|&l| l > 1) {
            (0..(N - l)).step_by(2 * l).for_each(|offset| {
                k += 1;
                let zeta = i32::from(ZETAS[k]);

                for j in offset..offset + l {
                    let temp = montgomery_reduce(zeta * i32::from(coeffs[j + l]));
                    coeffs[j + l] = coeffs[j] - temp;
                    coeffs[j] += temp;
                }
            });
        }

        Self {
            coeffs,
            state: self.state,
        }
    }
}

impl<S: State + Copy> Poly<S> {
    // In inverse NTT, with montgomery reduction
    // Assumes that all coefficients are bounded in absolute value by q.
    // If so, output coefficients are bounded in absolute value q.
    // If the input is in montgomery or regular form, then so is the output.
    // coefficients must be bounded in absolute value by 3713.
    // Example:
    // ```
    // let new_poly = poly.inv_ntt();
    // ```
    pub(crate) fn inv_ntt(&self) -> Self {
        let mut coeffs = self.coeffs;
        let mut k: usize = 127;

        for (l, reductions) in (1..)
            .map(|x| 1 << x)
            .zip(INV_NTT_REDUCTIONS)
            .take_while(|(l, _reductions)| *l < N)
        {
            (0..(N - 1)).step_by(2 * l).for_each(|offset| {
                let min_zeta = i32::from(ZETAS[k]);
                k -= 1;
                for j in offset..offset + l {
                    let temp = coeffs[j + l] - coeffs[j];
                    coeffs[j] += coeffs[j + l];
                    coeffs[j + l] = montgomery_reduce(min_zeta * i32::from(temp));
                }
            });
            for &i in reductions {
                coeffs[i] = barrett_reduce(coeffs[i]);
            }
        }

        for coeff in &mut coeffs {
            *coeff = montgomery_reduce(1441 * i32::from(*coeff));
        }

        Self {
            coeffs,
            state: self.state,
        }
    }
}
