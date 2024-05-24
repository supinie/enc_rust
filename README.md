![kyber_rust_ferris](./kyber_rust_ferris.png)

# enc_rust

[![codecov](https://codecov.io/github/supinie/enc_rust/branch/main/graph/badge.svg?token=S7UTUFQ8M5)](https://codecov.io/github/supinie/enc_rust)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![dependency status](https://deps.rs/repo/github/supinie/enc_rust/status.svg)](https://deps.rs/repo/github/supinie/enc_rust)

### About

A pure rust implementation of the Module-Lattice-based standards [ML-KEM](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.ipd.pdf) and [ML-DSA](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.ipd.pdf), also known as the PQC scheme [Crystals](https://pq-crystals.org/).

This package consists of a library (`enc_rust`), and (soon :TM:) a binary wrapper. The library currently contains implementations for ML-KEM (Kyber), and will in the future also support ML-DSA (Dilithium).

### Why enc_rust?

enc_rust aims to provide a secure, efficient, and ergonomic solution to any problem that requires quantum secure cryptography.

- No unsafe code
- `no_std` compatible
- ergonomic

enc_rust currently supports ML-KEM as a sole mechanism, but will provide:

- ML-KEM in hybrid with x25519
- ML-DSA
- ML-DSA in hybrid with Ed25519

### Disclaimer

This library and binary wrapper is offered as-is, and without a guarantee. Please exercise caution when using this library in a production application, and we accept no liability for any security issues related to the use of this code.

### Kyber Algorithm Authors:

- Roberto Avanzi, ARM Limited (DE)
- Joppe Bos, NXP Semiconductors (BE)
- Léo Ducas, CWI Amsterdam (NL)
- Eike Kiltz, Ruhr University Bochum (DE)
- Tancrède Lepoint, SRI International (US)
- Vadim Lyubashevsky, IBM Research Zurich (CH)
- John M. Schanck, University of Waterloo (CA)
- Peter Schwabe, MPI-SP (DE) & Radboud University (NL)
- Gregor Seiler, IBM Research Zurich (CH)
- Damien Stehle, ENS Lyon (FR)

