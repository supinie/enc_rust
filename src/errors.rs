use crate::params::{SecurityLevel, K};
use core::{
    fmt::{Display, Formatter},
    num::TryFromIntError,
};
use num_enum::TryFromPrimitiveError;

#[derive(Debug, PartialEq, Eq)]
pub enum CrystalsError {
    MismatchedSecurityLevels(SecurityLevel, SecurityLevel),
    IncorrectBufferLength(usize, usize),
}

impl Display for CrystalsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::MismatchedSecurityLevels(sec_level_1, sec_level_2) => write!(f, "Mismatched security levels when attempting operation: {sec_level_1:#?} and {sec_level_2:#?}"),
            Self::IncorrectBufferLength(buf_len, expected_buf_len) => write!(f, "Incorrect buffer length for (un)packing. Expected buffer length {expected_buf_len}, got length {buf_len}"),
        }
    }
}

pub enum PackingError {
    Crystals(CrystalsError),
    TryFromPrimitive(TryFromPrimitiveError<K>),
    TryFromInt(TryFromIntError),
}

impl From<CrystalsError> for PackingError {
    fn from(error: CrystalsError) -> Self {
        Self::Crystals(error)
    }
}

impl From<TryFromPrimitiveError<K>> for PackingError {
    fn from(error: TryFromPrimitiveError<K>) -> Self {
        Self::TryFromPrimitive(error)
    }
}

impl From<TryFromIntError> for PackingError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromInt(error)
    }
}
