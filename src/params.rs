use core::fmt::{Display, Formatter};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const N: usize = 256;

pub const Q: usize = 3329;
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const Q_I16: i16 = Q as i16;
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const Q_U16: u16 = Q as u16;
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const Q_I32: i32 = Q as i32;
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const Q_U32: u32 = Q as u32;

pub const Q_DIV: u64 = 80_635; // round(2^28 / Q)
pub const Q_DIV_VEC: u64 = 1_290_167; // round(2^32 / Q)

pub const SYMBYTES: usize = 32; // size of hashes

pub const SHAREDSECRETBYTES: usize = 32;

pub const POLYBYTES: usize = 384;

pub const MAX_CIPHERTEXT: usize = 1569;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
// Get the usize repr using .into()
// Convert from usize using K::try_from(my_usize)
pub enum K {
    Two = 2,
    #[default]
    Three = 3,
    Four = 4,
}

impl Display for K {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Two => write!(f, "512"),
            Self::Three => write!(f, "768"),
            Self::Four => write!(f, "1024"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive)]
#[repr(usize)]
// Get the usize repr using .into()
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
            K::Two => Self::FiveOneTwo {
                k,
                eta_1: Eta::Three,
                eta_2: Eta::Two,
            },
            K::Three => Self::SevenSixEight {
                k,
                eta_1: Eta::Two,
                eta_2: Eta::Two,
            },
            K::Four => Self::TenTwoFour {
                k,
                eta_1: Eta::Two,
                eta_2: Eta::Two,
            },
        }
    }

    pub const fn k(self) -> K {
        match self {
            Self::FiveOneTwo { k, .. }
            | Self::SevenSixEight { k, .. }
            | Self::TenTwoFour { k, .. } => k,
        }
    }

    pub const fn eta_1(self) -> Eta {
        match self {
            Self::FiveOneTwo { eta_1, .. }
            | Self::SevenSixEight { eta_1, .. }
            | Self::TenTwoFour { eta_1, .. } => eta_1,
        }
    }

    pub const fn eta_2(self) -> Eta {
        match self {
            Self::FiveOneTwo { eta_2, .. }
            | Self::SevenSixEight { eta_2, .. }
            | Self::TenTwoFour { eta_2, .. } => eta_2,
        }
    }

    pub const fn poly_compressed_bytes(self) -> usize {
        match self {
            Self::FiveOneTwo { .. } | Self::SevenSixEight { .. } => 128,
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
            Self::FiveOneTwo { k, .. } | Self::SevenSixEight { k, .. } => (k as usize) * 320,
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

    pub const fn ciphertext_bytes(self) -> usize {
        self.indcpa_bytes()
    }
}
