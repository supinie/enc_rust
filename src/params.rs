pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

#[derive(Debug, PartialEq)]
pub struct Params {
    pub k: usize,
    pub eta1: usize,
    pub eta2: usize,
}

impl Params {
    pub const fn sec_level_512() -> Self {
        Params {
            k: 2,
            eta1: 3,
            eta2: 2,
        }
    }

    pub const fn sec_level_768() -> Self {
        Params {
            k: 3,
            eta1: 2,
            eta2: 2,
        }
    }

    pub const fn sec_level_1024() -> Self {
        Params {
            k: 4,
            eta1: 2,
            eta2: 2,
        }
    }

    pub fn poly_compressed_bytes(&self) -> Option<usize> {
        match self.k {
            2 | 3 => Some(128),
            4 => Some(160),
            _ => None,
        }
    }

    pub fn poly_vec_bytes(&self) -> usize {
        self.k * POLYBYTES
    }

    pub fn poly_vec_compressed_bytes(&self) -> Option<usize> {
        match self.k {
            2 | 3 => Some(self.k * 320),
            4 => Some(self.k * 352),
            _ => None,
        }
    }

    pub fn indcpa_public_key_bytes(&self) -> usize {
        self.poly_vec_bytes() + SYMBYTES
    }

    pub fn indcpa_private_key_bytes(&self) -> usize {
        self.poly_vec_bytes()
    }

    pub fn indcpa_bytes(&self) -> usize {
        self.poly_vec_compressed_bytes().expect("invalid poly_vec_compressed_bytes") + self.poly_compressed_bytes().expect("invalid poly_compressed_bytes")
    }

    pub fn public_key_bytes(&self) -> usize {
        self.indcpa_public_key_bytes()
    }

    pub fn private_key_bytes(&self) -> usize {
        self.indcpa_private_key_bytes() + self.indcpa_public_key_bytes() + 2 * SYMBYTES
    }

    pub fn cipher_text_bytes(&self) -> usize {
        self.indcpa_bytes()
    }
}
