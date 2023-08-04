use more_asserts::assert_ge;

use crate::params::*;

const QPRIME: i32 = 62209;

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

pub const INV_NTT_REDUCTIONS: [i16; 79] = [
    -1,
	-1,
	16, 17, 48, 49, 80, 81, 112, 113, 144, 145, 176, 177, 208, 209, 240, 241, -1,
	0, 1, 32, 33, 34, 35, 64, 65, 96, 97, 98, 99, 128, 129, 160, 161, 162, 163, 192, 193, 224, 225, 226, 227, -1,
	2, 3, 66, 67, 68, 69, 70, 71, 130, 131, 194, 195, 196, 197, 198, 199, -1,
	4, 5, 6, 7, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, -1,
	-1
];

// given -2^15 q <= x < 2^15 q, returns -q < y < q with y = x 2^-16 mod q
pub fn montgomery_reduce(x: i32) -> i16 {
    let m = x.wrapping_mul(QPRIME) as i16;
    let t = (x - (m as i32).wrapping_mul(Q as i32)) >> 16;
    return t as i16;
}

// given x, return x 2^16 mod q
pub fn to_mont(x: i16) -> i16 {
    const R_SQUARED_MOD_Q: i32 = 1353;
    return montgomery_reduce((x as i32) * R_SQUARED_MOD_Q);
}

// given x, find 0 <= y <= q with y = x mod q
//
// iff x = -nq for some natural number n, barrett_reduce(x) = q != 0
pub fn barrett_reduce(x: i16) -> i16 {
    const APPROXIMATION: usize = 20159;
    // From Cloudflare's circl Kyber implementation:
    //
    // For any x we have x mod q = x - ⌊x/q⌋ q.  We will use 20159/2²⁶ as
	// an approximation of 1/q. Note that  0 ≤ 20159/2²⁶ - 1/q ≤ 0.135/2²⁶
	// and so | x 20156/2²⁶ - x/q | ≤ 2⁻¹⁰ for |x| ≤ 2¹⁶.  For all x
	// not a multiple of q, the number x/q is further than 1/q from any integer
	// and so ⌊x 20156/2²⁶⌋ = ⌊x/q⌋.  If x is a multiple of q and x is positive,
	// then x 20156/2²⁶ is larger than x/q so ⌊x 20156/2²⁶⌋ = ⌊x/q⌋ as well.
	// Finally, if x is negative multiple of q, then ⌊x 20156/2²⁶⌋ = ⌊x/q⌋-1.
	// Thus
	//                        [ q        if x=-nq for pos. integer n
	//  x - ⌊x 20156/2²⁶⌋ q = [
	//                        [ x mod q  otherwise
    let inside_floor = ((x as i32).wrapping_mul(APPROXIMATION as i32) >> 26) as i16;
    return x.wrapping_sub(inside_floor.wrapping_mul(Q as i16))
}

pub fn cond_sub_q(x: i16) -> i16 {
    assert_ge!(x, -29439, "x must be >= to -29439 when applying conditional subtract q");
    const Q_16: i16 = Q as i16;
    let mut result = x - Q_16;
    result += (result >> 15) & Q_16;
    return result
}
