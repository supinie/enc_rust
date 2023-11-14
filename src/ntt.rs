use crate::{field_operations::{barrett_reduce, montgomery_reduce}, params::N, polynomials::Poly};

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

#[rustfmt::skip]
const INV_NTT_REDUCTIONS: [i16; 79] = [
    -1,
	-1,
	16, 17, 48, 49, 80, 81, 112, 113, 144, 145, 176, 177, 208, 209, 240, 241, -1,
	0, 1, 32, 33, 34, 35, 64, 65, 96, 97, 98, 99, 128, 129, 160, 161, 162, 163, 192, 193, 224, 225, 226, 227, -1,
	2, 3, 66, 67, 68, 69, 70, 71, 130, 131, 194, 195, 196, 197, 198, 199, -1,
	4, 5, 6, 7, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, -1,
	-1
];

impl Poly {
    // In place Cooley-Tukey radix-2 Decimation in Time (DIT) NTT algorithm
    // Example:
    // poly.ntt();
    pub(crate) fn ntt(&mut self) {
        let mut k = 0usize;
        let mut l = N / 2;
        while l > 1 {
            let mut offset = 0;
            while offset < N - l {
                k += 1;
                let zeta = i32::from(ZETAS[k]);

                let mut j = offset;
                while j < offset + l {
                    let t = montgomery_reduce(zeta * i32::from(self.coeffs[j + l]));
                    self.coeffs[j + l] = self.coeffs[j] - t;
                    self.coeffs[j] += t;
                    j += 1;
                }
                offset += 2 * l;
            }
            l >>= 1;
        }
    }

    // In place inverse NTT, with montgomery reduction
    // Example:
    // poly.inv_ntt();
    pub(crate) fn inv_ntt(&mut self) {
        let mut k: usize = 127;
        let mut r: usize = 0;
        let mut l = 2;

        while l < N {
            for offset in (0..N - 1).step_by(2 * l) {
                let min_zeta = i32::from(ZETAS[k]);
                k -= 1;

                for j in offset..offset + l {
                    let t = self.coeffs[j + l] - self.coeffs[j];
                    self.coeffs[j] += self.coeffs[j + l];
                    self.coeffs[j + l] = montgomery_reduce(min_zeta * i32::from(t));
                }
            }

            #[allow(clippy::cast_sign_loss)] // i cannot be negative if we reach where its value is
                                             // cast
            loop {
                let i = INV_NTT_REDUCTIONS[r];
                r += 1;
                if i < 0 {
                    break;
                }
                self.coeffs[i as usize] = barrett_reduce(self.coeffs[i as usize]);
            }
            l <<= 1;
        }

        for j in 0..N {
            self.coeffs[j] = montgomery_reduce(1441 * i32::from(self.coeffs[j]));
        }
    }
}
