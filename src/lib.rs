#![allow(unused)]
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

//! ### About

//! A pure rust implementation of the Module-Lattice-based standards [ML-KEM](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.ipd.pdf) and (soon) [ML-DSA](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.ipd.pdf), also known as the PQC scheme [Crystals](https://pq-crystals.org/).
//!
//! This package consists of a library (`enc_rust`), and (soon :TM:) a binary wrapper. The library currently contains implementations for ML-KEM (Kyber), and will in the future also support ML-DSA (Dilithium).
//!
//! ---
//!
//! ### Why enc_rust?
//!
//! enc_rust aims to provide a secure, efficient, and ergonomic solution to any problem that requires quantum secure cryptography.
//!
//! - No unsafe code
//! - `no_std` compatible
//! - ergonomic
//!
//! enc_rust currently supports ML-KEM as a sole mechanism, but will provide:
//!
//! - ML-KEM in hybrid with x25519
//! - ML-DSA
//! - ML-DSA in hybrid with Ed25519
//!
//! ---
//!
//! ### How to use
//!
//! Currently, enc_rust is not released as a crate, but plans to launch soon. In its current state, it can be used with `cargo add --git https://github.com/supinie/enc_rust`.
//!
//! #### Example
//!
//! ```rust
//! use enc_rust::kem::*;
//!
//! fn alice(pk: PublicKey) -> (Ciphertext, [u8; 32]) {
//!     let (ciphertext, shared_secret) = pk.encapsulate(None, None).unwrap();
//!
//!     (ciphertext, shared_secret)
//! }
//!
//! fn bob(sk: PrivateKey, ciphertext: &[u8]) -> [u8; 32] {
//!     let shared_secret = sk.decapsulate(ciphertext).unwrap();
//!
//!     shared_secret
//! }
//!
//!
//! fn main() {
//!     let (pk, sk) = generate_key_pair(None, 3).unwrap();
//!
//!     let (ciphertext, alice_secret) = alice(pk);
//!
//!     let bob_secret = bob(sk, ciphertext.as_bytes());
//!
//!     assert_eq!(alice_secret, bob_secret);
//! }
//! ```
//!
//! ### Disclaimer
//!
//! This library and binary wrapper is offered as-is, and without a guarantee. Please exercise caution when using this library in a production application, and we accept no liability for any security issues related to the use of this code.
//!
//! ---
//!
//! ### Kyber Algorithm Authors:
//!
//! - Roberto Avanzi, ARM Limited (DE)
//! - Joppe Bos, NXP Semiconductors (BE)
//! - Léo Ducas, CWI Amsterdam (NL)
//! - Eike Kiltz, Ruhr University Bochum (DE)
//! - Tancrède Lepoint, SRI International (US)
//! - Vadim Lyubashevsky, IBM Research Zurich (CH)
//! - John M. Schanck, University of Waterloo (CA)
//! - Peter Schwabe, MPI-SP (DE) & Radboud University (NL)
//! - Gregor Seiler, IBM Research Zurich (CH)
//! - Damien Stehle, ENS Lyon (FR)

pub mod errors;
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
    mod kem;
    mod matrix;
    mod ntt;
    mod params;
    mod polynomials;
    mod sample;
    mod vectors;
}
