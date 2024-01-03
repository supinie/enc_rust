use num_enum::IntoPrimitive;

pub const N: usize = 256;
pub const Q: usize = 3329;

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
// Get the u8 repr using .into()
pub enum K {
    Two = 2,
    Three = 3,
    Four = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum Eta {
    Two = 2,
    Three = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecurityLevel {
    FiveOneTwo { k: K, eta_1: Eta, eta_2: Eta },
    SevenSixEight { k: K, eta_1: Eta, eta_2: Eta },
    TenTwoFour { k: K, eta_1: Eta, eta_2: Eta },
}

impl SecurityLevel {
    pub const fn new(k: K) -> Self {
        match k {
            K::Two => Self::FiveOneTwo { k, eta_1: Eta::Three, eta_2: Eta::Two },
            K::Three => Self::SevenSixEight { k, eta_1: Eta::Two, eta_2: Eta::Two },
            K::Four => Self::TenTwoFour { k, eta_1: Eta::Two, eta_2: Eta::Two },
        }
    }

    pub const fn poly_compressed_bytes(self) -> usize {
        match self {
            Self::FiveOneTwo { .. }
            | Self::SevenSixEight { .. } => 128,
            Self::TenTwoFour { .. } => 160,
        }
    }

    pub const fn poly_vec_bytes(self) -> usize {
        match self {
            Self::FiveOneTwo { k, .. }
            | Self::SevenSixEight { k, .. }
            | Self::TenTwoFour { k, .. } => (k as usize) * POLYBYTES,
        }
    }

    pub const fn poly_vec_compressed_bytes(self) -> usize {
        match self {
            Self::FiveOneTwo { k, .. }
            | Self::SevenSixEight { k, .. } => (k as usize) * 320,
            Self::TenTwoFour { k, .. } => (k as usize) * 352,
        }
    }

    pub const fn indcpa_public_key_bytes(self) -> usize {
        self.poly_vec_bytes() + SYMBYTES
    }

    pub const fn indcpa_private_key_bytes(self) -> usize {
        self.poly_vec_bytes()
    }

    pub const fn indcpa_bytes(self) -> usize {
        self.poly_vec_compressed_bytes() + self.poly_compressed_bytes()
    }

    pub const fn public_key_bytes(self) -> usize {
        self.indcpa_public_key_bytes()
    }

    pub const fn private_key_bytes(self) -> usize {
        self.indcpa_private_key_bytes() + self.indcpa_public_key_bytes() + 2 * SYMBYTES
    }

    pub const fn cipher_text_bytes(self) -> usize {
        self.indcpa_bytes()
    }
}
