use crate::params::K;
use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum CrystalsError {
    MismatchedSecurityLevels(K, K),
}

impl Display for CrystalsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::MismatchedSecurityLevels(sec_level_1, sec_level_2) => write!(f, "Mismatched security levels when attempting operation: {sec_level_1:#?} and {sec_level_2:#?}"),
        }
    }
}
