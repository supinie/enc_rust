use crate::{
    params::{GetSecLevel, SecurityLevel, K},
    vectors::{PolyVec1024, PolyVec512, PolyVec768},
};

pub type Mat512 = [PolyVec512; 2];
pub type Mat768 = [PolyVec768; 3];
pub type Mat1024 = [PolyVec1024; 4];

pub trait Operations {
    // seed length 32
    fn derive(&mut self, seed: &[u8], transpose: bool);
    fn transpose(&mut self);
}

impl GetSecLevel for Mat512 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Two)
    }
}

impl GetSecLevel for Mat768 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Three)
    }
}

impl GetSecLevel for Mat1024 {
    fn sec_level() -> SecurityLevel {
        SecurityLevel::new(K::Four)
    }
}

macro_rules! impl_matrix {
    ($variant:ty) => {
        impl Operations for $variant {
            fn derive(&mut self, seed: &[u8], transpose: bool) {
                match transpose {
                    true => {
                        for (i, vector) in self.iter_mut().enumerate() {
                            for (j, polynomial) in vector.iter_mut().enumerate() {
                                polynomial.derive_uniform(seed, i as u8, j as u8);
                            }
                        }
                    }
                    false => {
                        for (i, vector) in self.iter_mut().enumerate() {
                            for (j, polynomial) in vector.iter_mut().enumerate() {
                                polynomial.derive_uniform(seed, j as u8, i as u8);
                            }
                        }
                    }
                }
            }

            fn transpose(&mut self) {
                let k = <$variant as GetSecLevel>::sec_level().k().into();
                for i in 0..k - 1 {
                    for j in i + 1..k {
                        let temp = self[i][j];
                        self[i][j] = self[j][i];
                        self[j][i] = temp;
                    }
                }
            }
        }
    };
}

impl_matrix!(Mat512);
impl_matrix!(Mat768);
impl_matrix!(Mat1024);
