use crate::params::{SecurityLevel, K};
use core::{
    array::TryFromSliceError,
    fmt::{Display, Formatter},
    num::TryFromIntError,
};
use num_enum::TryFromPrimitiveError;

#[derive(Debug, PartialEq, Eq)]
pub enum CrystalsError {
    MismatchedSecurityLevels(SecurityLevel, SecurityLevel),
    IncorrectBufferLength(usize, usize),
    InvalidSeedLength(usize, usize),
    InternalError(),
    InvalidK(usize),
    InvalidCiphertextLength(usize),
}

impl Display for CrystalsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::MismatchedSecurityLevels(sec_level_1, sec_level_2) => write!(f, "Mismatched security levels when attempting operation: {sec_level_1:#?} and {sec_level_2:#?}"),
            Self::IncorrectBufferLength(buf_len, expected_buf_len) => write!(f, "Incorrect buffer length for (un)packing. Expected buffer length {expected_buf_len}, got length {buf_len}"),
            Self::InvalidSeedLength(seed_len, expected_seed_len) => write!(f, "Invalid seed length, expected {expected_seed_len}, got {seed_len}"),
            Self::InternalError() => write!(f, "Unexpected internal error"),
            Self::InvalidK(k) => write!(f, "Recieved invalid k value, {k}, expected 2, 3, or 4"),
            Self::InvalidCiphertextLength(ciphertext_len) => write!(f, "Invalid ciphertext length, expected 768, 1088, or 1568, got {ciphertext_len}"),
        }
    }
}

#[derive(Debug)]
pub enum PackingError {
    Crystals(CrystalsError),
    TryFromPrimitive(TryFromPrimitiveError<K>),
    TryFromInt(TryFromIntError),
    TryFromSlice(TryFromSliceError),
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

impl From<TryFromSliceError> for PackingError {
    fn from(error: TryFromSliceError) -> Self {
        Self::TryFromSlice(error)
    }
}

#[derive(Debug)]
pub enum KeyGenerationError {
    Crystals(CrystalsError),
    TryFromSlice(TryFromSliceError),
    Packing(PackingError),
    Rand(rand_core::Error),
}

impl From<CrystalsError> for KeyGenerationError {
    fn from(error: CrystalsError) -> Self {
        Self::Crystals(error)
    }
}

impl From<TryFromSliceError> for KeyGenerationError {
    fn from(error: TryFromSliceError) -> Self {
        Self::TryFromSlice(error)
    }
}

impl From<PackingError> for KeyGenerationError {
    fn from(error: PackingError) -> Self {
        Self::Packing(error)
    }
}

impl From<rand_core::Error> for KeyGenerationError {
    fn from(error: rand_core::Error) -> Self {
        Self::Rand(error)
    }
}

#[derive(Debug)]
pub enum EncryptionDecryptionError {
    Crystals(CrystalsError),
    KeyGenerationError(KeyGenerationError),
    TryFromInt(TryFromIntError),
    Packing(PackingError),
    Rand(rand_core::Error),
}

impl From<CrystalsError> for EncryptionDecryptionError {
    fn from(error: CrystalsError) -> Self {
        Self::Crystals(error)
    }
}

impl From<KeyGenerationError> for EncryptionDecryptionError {
    fn from(error: KeyGenerationError) -> Self {
        Self::KeyGenerationError(error)
    }
}

impl From<TryFromIntError> for EncryptionDecryptionError {
    fn from(error: TryFromIntError) -> Self {
        Self::TryFromInt(error)
    }
}

impl From<PackingError> for EncryptionDecryptionError {
    fn from(error: PackingError) -> Self {
        Self::Packing(error)
    }
}

impl From<rand_core::Error> for EncryptionDecryptionError {
    fn from(error: rand_core::Error) -> Self {
        Self::Rand(error)
    }
}
