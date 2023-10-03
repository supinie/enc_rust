use crate::poly::*;

#[derive(Clone, PartialEq)]
pub enum PolyVec {
    PolyVec512([Poly; 2]),
    PolyVec768([Poly; 3]),
    PolyVec1024([Poly; 4]),
}

impl PolyVec {
    pub fn len(&self) -> usize {
        match self {
            PolyVec::PolyVec512(_) => 2,
            PolyVec::PolyVec768(_) => 3,
            PolyVec::PolyVec1024(_) => 4,
        }
    }


    pub fn new(poly_array: &[Poly]) -> Option<Self>  {
        match poly_array.len() {
            2 => Some(PolyVec::PolyVec512(poly_array.try_into().expect("invalid poly array"))),
            3 => Some(PolyVec::PolyVec768(poly_array.try_into().expect("invalid poly array"))),
            4 => Some(PolyVec::PolyVec1024(poly_array.try_into().expect("invalid poly array"))),
            _ => None,
        }
    }

    pub fn polys_mut(&mut self) -> &mut [Poly] {
        match self {
            PolyVec::PolyVec512(ref mut polys) => polys,
            PolyVec::PolyVec768(ref mut polys) => polys,
            PolyVec::PolyVec1024(ref mut polys) => polys,
        }
    }

    pub fn polys(&self) -> &[Poly] {
        match self {
            PolyVec::PolyVec512(ref polys) => polys,
            PolyVec::PolyVec768(ref polys) => polys,
            PolyVec::PolyVec1024(ref polys) => polys,
        }
    }
            
    // Adds the given vector of polynomial and sets self to be the sum
    // Example:
    // vec1.add(vec2);
    pub fn add(&mut self, x: &PolyVec) {
        assert_eq!(self.len(), x.len());
        for i in 0..self.len() {
            self.polys_mut()[i].add(&x.polys()[i]);
        }
    }

    pub fn reduce(&mut self) {
        for i in 0..self.len() {
            self.polys_mut()[i].reduce();
        }
    }

    pub fn normalise(&mut self) {
        for i in 0..self.len() {
            self.polys_mut()[i].normalise();
        }
    }

    pub fn ntt(&mut self) {
        for i in 0..self.len() {
            self.polys_mut()[i].ntt();
        }
    }

    pub fn inv_ntt(&mut self) {
        for i in 0..self.len() {
            self.polys_mut()[i].inv_ntt();
        }
    }
}
