pub const K: usize = if cfg!(feature = "kyber512") {
    2
} else if cfg!(feature = "kyber1024") {
    4
} else {
    3
};

pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;
// pub const POLYVECBYTES: usize = 
