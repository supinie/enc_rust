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
        for poly in self.polys_mut().iter_mut() {
            poly.reduce();
        }
    }

    pub fn normalise(&mut self) {
        for poly in self.polys_mut().iter_mut() {
            poly.normalise();
        }
    }

    pub fn ntt(&mut self) {
        for poly in self.polys_mut().iter_mut() {
            poly.ntt();
        }
    }

    pub fn inv_ntt(&mut self) {
        for poly in self.polys_mut().iter_mut() {
            poly.inv_ntt();
        }
    }

    pub fn inner_product_pointwise(&mut self, a: &PolyVec, b: &PolyVec) {
        todo!();
    }

    pub fn derive_noise(&mut self, seed: &[u8], nonce: u8, eta: usize) -> Result<(), &str> {
        for poly in self.polys_mut().iter_mut() {
            poly.derive_noise(seed, nonce, eta)?;
        }
        Ok(())
    }
}
