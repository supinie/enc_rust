use more_asserts::assert_ge;

use crate::params::*;

// given -2^15 q <= x < 2^15 q, returns -q < y < q with y = x 2^-16 mod q
pub fn montgomery_reduce(x: i32) -> i16 {
    const QPRIME: i32 = 62209;
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
    return x.wrapping_sub(inside_floor.wrapping_mul(Q as i16));
}

pub fn cond_sub_q(x: i16) -> i16 {
    assert_ge!(
        x,
        -29439,
        "x must be >= to -29439 when applying conditional subtract q"
    );
    const Q_16: i16 = Q as i16;
    let mut result = x - Q_16;
    result += (result >> 15) & Q_16;
    return result;
}
