pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

pub const POLYCOMPRESSEDBYTES: usize = 128;

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

    pub fn poly_compressed_bytes(&self) -> usize {
        match self.k {
            2 | 3 => 128,
            4 => 160,
            _ => panic!("Invalid security level passed to poly_compressed_bytes"),
        }
    }

    pub fn poly_vec_bytes(&self) -> usize {
        self.k * POLYBYTES
    }

    pub fn poly_vec_compressed_bytes(&self) -> usize {
        match self.k {
            2 | 3 => self.k * 320,
            4 => self.k * 352,
            _ => panic!("Invalid security level passed to poly_vec_compressed_bytes"),
        }
    }

    pub fn indcpa_public_key_bytes(&self) -> usize {
        self.poly_vec_bytes() + SYMBYTES
    }

    pub fn indcpa_private_key_bytes(&self) -> usize {
        self.poly_vec_bytes()
    }

    pub fn indcpa_bytes(&self) -> usize {
        self.poly_vec_compressed_bytes() + self.poly_compressed_bytes()
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
