use crate::{
    errors::CrystalsError,
    params::{SecurityLevel, K},
    polynomials::{Montgomery, Poly, State},
    vectors::PolyVec,
};
use tinyvec::ArrayVec;

#[derive(Default, PartialEq, Debug, Eq)]
pub struct Matrix<S: State> {
    polyvecs: ArrayVec<[PolyVec<S>; 4]>,
    sec_level: K,
}

impl<S: State + Copy> Matrix<S> {
    pub(crate) const fn sec_level(&self) -> SecurityLevel {
        SecurityLevel::new(self.sec_level)
    }

    pub(crate) fn vectors(&self) -> &[PolyVec<S>] {
        &self.polyvecs.as_slice()[..self.sec_level.into()]
    }

    pub(crate) fn transpose(&self) -> Result<Self, CrystalsError> {
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

impl Matrix<Montgomery> {
    pub(crate) fn derive(
        seed: &[u8],
        transpose: bool,
        sec_level: K,
    ) -> Result<Self, CrystalsError> {
        let mut polyvecs = ArrayVec::<[PolyVec<Montgomery>; 4]>::new();
        if transpose {
            for i in 0..sec_level.into() {
                let row: ArrayVec<[Poly<Montgomery>; 4]> = (0..sec_level.into())
                    .map(|j| {
                        #[allow(clippy::cast_possible_truncation)] // we know that max i, j is 4
                        Poly::derive_uniform(seed, i as u8, j as u8)
                    })
                    .collect::<Result<ArrayVec<[Poly<Montgomery>; 4]>, CrystalsError>>()?;

                let polyvec = PolyVec::from(row)?;
                polyvecs.push(polyvec);
            }
        } else {
            for i in 0..sec_level.into() {
                let row: ArrayVec<[Poly<Montgomery>; 4]> = (0..sec_level.into())
                    .map(|j| {
                        #[allow(clippy::cast_possible_truncation)] // we know that max i, j is 4
                        Poly::derive_uniform(seed, j as u8, i as u8)
                    })
                    .collect::<Result<ArrayVec<[Poly<Montgomery>; 4]>, CrystalsError>>()?;

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
