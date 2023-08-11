pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

pub const POLYCOMPRESSEDBYTES: usize = 128;

pub struct Params {
    pub k: usize,
    pub eta1: usize,
    pub eta2: usize,
}

impl Params {
    pub fn poly_vec_bytes(&self) -> usize {
        self.k * POLYBYTES
    }

    pub fn poly_vec_compressed_bytes(&self) -> usize {
        self.k * 320
    }

    pub fn indcpa_public_key_bytes(&self) -> usize {
        self.poly_vec_bytes() + SYMBYTES
    }

    pub fn indcpa_private_key_bytes(&self) -> usize {
        self.poly_vec_bytes()
    }

    pub fn indcpa_bytes(&self) -> usize {
        self.poly_vec_compressed_bytes() + POLYCOMPRESSEDBYTES
    }

    pub fn public_key_bytes(&self) -> usize {
        self.indcpa_public_key_bytes()
    }

    pub fn private_key_bytes(&self) -> usize {
        self.indcpa_private_key_bytes() + self.indcpa_private_key_bytes() + 2 * SYMBYTES
    }

    pub fn cipher_text_bytes(&self) -> usize {
        self.indcpa_bytes()
    }
}

pub fn set_params(sec_level: usize) -> Params {
    Params {
        k: sec_level,
        eta1: {
            if sec_level == 2 {
                3
            } else {
                2
            }
        },
        eta2: 2,
    }
}
