// REMOVE BEFORE FINAL, DEVELOPMENT ONLY
#![allow(unused)]
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
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

mod buffer;
mod field_ops;
mod indcpa;
pub mod kem;
mod kex;
mod ntt;
pub mod params;
mod poly;
mod vec;
mod sample;

mod tests {
    mod buffer;
    mod field;
    mod ntt;
    mod params;
    mod poly;
    mod vec;
    mod sample;
}
