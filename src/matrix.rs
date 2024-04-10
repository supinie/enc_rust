use crate::{
    errors::CrystalsError,
    params::{SecurityLevel, K},
    polynomials::{Normalised, Poly, State, Unnormalised},
    vectors::PolyVec,
};
use tinyvec::{array_vec, ArrayVec};

#[derive(Default, PartialEq, Debug, Eq)]
pub struct Matrix<S: State> {
    polyvecs: ArrayVec<[PolyVec<S>; 4]>,
    sec_level: K,
}

impl<S: State + Copy> Matrix<S> {
    const fn sec_level(&self) -> SecurityLevel {
        SecurityLevel::new(self.sec_level)
    }

    fn vectors(&self) -> &[PolyVec<S>] {
        &self.polyvecs.as_slice()[..self.sec_level.into()]
    }

    fn transpose(&self) -> Result<Self, CrystalsError> {
        let mut raw_matrix = [ArrayVec::<[Poly<S>; 4]>::new(); 4];
        self.vectors()
            .iter()
            .flat_map(|vec| vec.polynomials().iter().enumerate())
            .for_each(|(i, poly)| {
                raw_matrix[i].push(*poly);
            });

        let polyvecs_result: Result<ArrayVec<[PolyVec<S>; 4]>, CrystalsError> = raw_matrix
            [..self.sec_level.into()]
            .iter()
            .map(|vec| PolyVec::from(*vec))
            .collect::<Result<ArrayVec<[PolyVec<S>; 4]>, CrystalsError>>();

        match polyvecs_result {
            Ok(polyvecs) => Ok(Self {
                polyvecs,
                sec_level: self.sec_level,
            }),
            Err(err) => Err(err),
        }
    }
}

impl Matrix<Normalised> {
    // Create a new, empty matrix
    fn new(k: K) -> Self {
        let polyvecs = match k {
            K::Two => array_vec!([PolyVec<Normalised>; 4] => PolyVec::new(k), PolyVec::new(k)),
            K::Three => {
                array_vec!([PolyVec<Normalised>; 4] => PolyVec::new(k), PolyVec::new(k), PolyVec::new(k))
            }
            K::Four => {
                array_vec!([PolyVec<Normalised>; 4] => PolyVec::new(k), PolyVec::new(k), PolyVec::new(k), PolyVec::new(k))
            }
        };

        Self {
            polyvecs,
            sec_level: k,
        }
    }
}

impl Matrix<Unnormalised> {
    fn derive(seed: &[u8], transpose: bool, sec_level: K) -> Result<Self, CrystalsError> {
        let mut polyvecs = ArrayVec::<[PolyVec<Unnormalised>; 4]>::new();
        if transpose {
            for i in 0..sec_level.into() {
                let row: ArrayVec<[Poly<Unnormalised>; 4]> = (0..sec_level.into())
                    .map(|j| {
                        #[allow(clippy::cast_possible_truncation)] // we know that max i, j is 4
                        Poly::derive_uniform(seed, i as u8, j as u8)
                    })
                    .collect::<Result<ArrayVec<[Poly<Unnormalised>; 4]>, CrystalsError>>()?;

                let polyvec = PolyVec::from(row)?;
                polyvecs.push(polyvec);
            }
        } else {
            for i in 0..sec_level.into() {
                let row: ArrayVec<[Poly<Unnormalised>; 4]> = (0..sec_level.into())
                    .map(|j| {
                        #[allow(clippy::cast_possible_truncation)] // we know that max i, j is 4
                        Poly::derive_uniform(seed, j as u8, i as u8)
                    })
                    .collect::<Result<ArrayVec<[Poly<Unnormalised>; 4]>, CrystalsError>>()?;

                let polyvec = PolyVec::from(row)?;
                polyvecs.push(polyvec);
            }
        }

        Ok(Self {
            polyvecs,
            sec_level,
        })
    }
}

// pub type Mat512 = [PolyVec512; 2];
// pub type Mat768 = [PolyVec768; 3];
// pub type Mat1024 = [PolyVec1024; 4];

// pub trait New {
//     fn new() -> Self;
// }

// pub trait MatOperations {
//     // seed length 32
//     fn derive(seed: &[u8], transpose: bool) -> Self;
//     fn transpose(&mut self);
// }

// impl New for Mat512 {
//     fn new() -> Self {
//         [PolyVec512::from([Poly::new(); 2]); 2]
//     }
// }

// impl New for Mat768 {
//     fn new() -> Self {
//         [PolyVec768::from([Poly::new(); 3]); 3]
//     }
// }

// impl New for Mat1024 {
//     fn new() -> Self {
//         [PolyVec1024::from([Poly::new(); 4]); 4]
//     }
// }

// impl GetSecLevel for Mat512 {
//     fn sec_level() -> SecurityLevel {
//         SecurityLevel::new(K::Two)
//     }
// }

// impl GetSecLevel for Mat768 {
//     fn sec_level() -> SecurityLevel {
//         SecurityLevel::new(K::Three)
//     }
// }

// impl GetSecLevel for Mat1024 {
//     fn sec_level() -> SecurityLevel {
//         SecurityLevel::new(K::Four)
//     }
// }

// macro_rules! impl_matrix {
//     ($variant:ty) => {
//         impl MatOperations for $variant {
//             fn derive(seed: &[u8], transpose: bool) -> Self {
//                 let mut matrix = Self::new();
//                 match transpose {
//                     true => {
//                         for (i, vector) in matrix.iter_mut().enumerate() {
//                             for (j, polynomial) in vector.iter_mut().enumerate() {
//                                 polynomial.derive_uniform(seed, i as u8, j as u8);
//                             }
//                         }
//                     }
//                     false => {
//                         for (i, vector) in matrix.iter_mut().enumerate() {
//                             for (j, polynomial) in vector.iter_mut().enumerate() {
//                                 polynomial.derive_uniform(seed, j as u8, i as u8);
//                             }
//                         }
//                     }
//                 }
//                 matrix
//             }

//             fn transpose(&mut self) {
//                 let k: u8 = <$variant as GetSecLevel>::sec_level().k().into();
//                 for i in 0..usize::from(k - 1) {
//                     for j in i + 1..usize::from(k) {
//                         let temp = self[i][j];
//                         self[i][j] = self[j][i];
//                         self[j][i] = temp;
//                     }
//                 }
//             }
//         }
//     };
// }

// impl_matrix!(Mat512);
// impl_matrix!(Mat768);
// impl_matrix!(Mat1024);
