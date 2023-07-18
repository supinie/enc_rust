pub const K_512: usize = 2;
pub const K_768: usize = 3;
pub const K_1024: usize = 4;

pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

pub const POLYCOMPRESSEDBYTES: usize = 128;

pub trait ParamsTemplate {
    const K: usize;
    const ETA: usize;
    const POLYVECBYTES: usize;
    const POLYVECCOMPRESSEDBYTES: usize;
    const INDCPAPUBLICKEYBYTES: usize;
    const INDCPASECRETKEYBYTES: usize;
    const INDCPABYTES: usize;
    const PUBLICKEYBYTES: usize;
    const SECRETKEYBYTES: usize;
    const CIPHERTEXTBYTES: usize;
}

pub struct Params;

// pub fn set_512_params() {
//     impl ParamsTemplate for Params {
//         const K: usize = K_512;
//         const ETA: usize = 5;
//         const POLYVECBYTES: usize = Self::K * POLYBYTES;
//         const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
//         const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
//         const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
//         const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
//         const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
//         const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
//         const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
//     }
// }

// pub fn set_768_params() {
//     impl ParamsTemplate for Params {
//         const K: usize = K_768;
//         const ETA: usize = 5;
//         const POLYVECBYTES: usize = Self::K * POLYBYTES;
//         const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
//         const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
//         const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
//         const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
//         const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
//         const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
//         const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
//     }
// }

// pub fn set_1024_params() {
//     impl ParamsTemplate for Params {
//         const K: usize = K_1024;
//         const ETA: usize = 5;
//         const POLYVECBYTES: usize = Self::K * POLYBYTES;
//         const POLYVECCOMPRESSEDBYTES: usize = Self::K * 320;
//         const INDCPAPUBLICKEYBYTES: usize = Self::POLYVECBYTES + SYMBYTES;
//         const INDCPASECRETKEYBYTES: usize = Self::POLYVECBYTES;
//         const INDCPABYTES: usize = Self::POLYVECCOMPRESSEDBYTES + POLYCOMPRESSEDBYTES;
//         const PUBLICKEYBYTES: usize = Self::INDCPAPUBLICKEYBYTES;
//         const SECRETKEYBYTES: usize = Self::INDCPASECRETKEYBYTES + Self::INDCPAPUBLICKEYBYTES + 2 * SYMBYTES;
//         const CIPHERTEXTBYTES: usize = Self::INDCPABYTES;
//     }
// }
