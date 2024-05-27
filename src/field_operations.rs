use crate::params::{Q_I16, Q_I32};

// given -2^15 q <= x < 2^15 q, returns -q < y < q with y congruent to x * 2^-16 mod q
// Example:
// ```
// let x = montgomery_reduce(5);
// ```
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn montgomery_reduce(x: i32) -> i16 {
    const QPRIME: i32 = 62209;
    let m = x.wrapping_mul(QPRIME) as i16;
    let t = (x - i32::from(m).wrapping_mul(Q_I32)) >> 16;
    t as i16
}

// given x, return x 2^16 mod q
// Example:
// let x = mont_form(y);
pub fn mont_form(x: i16) -> i16 {
    const R_SQUARED_MOD_Q: i32 = 1353;
    montgomery_reduce(i32::from(x) * R_SQUARED_MOD_Q)
}

// given x, find 0 <= y <= q with y = x mod q
//
// iff x = -nq for some natural number n, barrett_reduce(x) = q != 0
// Example:
// let x = barrett_reduce(y);
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn barrett_reduce(x: i16) -> i16 {
    const APPROXIMATION: i32 = 20159;
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
    let inside_floor = (i32::from(x).wrapping_mul(APPROXIMATION) >> 26) as i16;
    x.wrapping_sub(inside_floor.wrapping_mul(Q_I16))
}

// given x, if x < Q return x, otherwise return x - Q
// Example:
// let x = conditional_sub_q(y);
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const fn conditional_sub_q(x: i16) -> i16 {
    const Q_16: i16 = Q_I16;
    if x < Q_16 {
        x
    } else {
        let mut result = x - Q_16;
        result += (result >> 15) & Q_16;
        result
    }
}
