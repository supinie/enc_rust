// REMOVE BEFORE FINAL, DEVELOPMENT ONLY
// #![allow(unused)]
#![allow(dead_code)]
// REMOVE BEFORE FINAL, DEVELOPMENT ONLY
#![forbid(unsafe_code)]
#![warn(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::implicit_saturating_sub,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![no_std]
// #![allow(clippy::needless_range_loop)]

mod errors;
mod field_operations;
mod indcpa;
pub mod kem;
mod matrix;
pub(crate) mod params;
mod polynomials;
mod vectors;

mod tests {
    // mod buffer;
    mod field_operations;
    mod indcpa;
    mod matrix;
    mod ntt;
    mod sample;
    mod params;
    mod polynomials;
    mod vectors;
}
