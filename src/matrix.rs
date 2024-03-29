use crate::{
    params::{GetSecLevel, SecurityLevel, K},
    polynomials::Poly,
    vectors::{PolyVec1024, PolyVec512, PolyVec768},
};
use tinyvec::ArrayVec;

pub type Mat512 = [PolyVec512; 2];
pub type Mat768 = [PolyVec768; 3];
pub type Mat1024 = [PolyVec1024; 4];

pub trait New {
    fn new() -> Self;
}

pub trait MatOperations {
    // seed length 32
    fn derive(seed: &[u8], transpose: bool) -> Self;
    fn transpose(&mut self);
}

impl New for Mat512 {
    fn new() -> Self {
        [PolyVec512::from([Poly::new(); 2]); 2]
    }
}

impl New for Mat768 {
    fn new() -> Self {
        [PolyVec768::from([Poly::new(); 3]); 3]
    }
}

impl New for Mat1024 {
    fn new() -> Self {
        [PolyVec1024::from([Poly::new(); 4]); 4]
    }
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
        impl MatOperations for $variant {
            fn derive(seed: &[u8], transpose: bool) -> Self {
                let mut matrix = Self::new();
                match transpose {
                    true => {
                        for (i, vector) in matrix.iter_mut().enumerate() {
                            for (j, polynomial) in vector.iter_mut().enumerate() {
                                polynomial.derive_uniform(seed, i as u8, j as u8);
                            }
                        }
                    }
                    false => {
                        for (i, vector) in matrix.iter_mut().enumerate() {
                            for (j, polynomial) in vector.iter_mut().enumerate() {
                                polynomial.derive_uniform(seed, j as u8, i as u8);
                            }
                        }
                    }
                }
                matrix
            }

            fn transpose(&mut self) {
                let k: u8 = <$variant as GetSecLevel>::sec_level().k().into();
                for i in 0..usize::from(k - 1) {
                    for j in i + 1..usize::from(k) {
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
