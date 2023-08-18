use crate::{
    poly::*,
    params::*,
    ntt::*,
}

#[derive(Copy, Clone)]
pub struct PolyVec {
    pub polynomials: Vec<Poly>,
    pub params: Params,
}

impl PolyVec {

    // Adds the given vector of polynomial and sets self to be the sum
    // Example:
    // vec1.add(vec2);
    pub fn add(&mut self, x: &PolyVec) {
        for i in 0..self.params.k {
            self.polynomials[i].add(x.polynomials[i]);
        }
    }

    pub fn reduce(&mut self) {
        for i in 0..self.params.k {
            self.polynomials[i].reduce();
        }
    }

    pub fn normalise(&mut self) {
        for i in 0..self.params.k {
            self.polynomials[i].reduce();
        }
    }

    pub fn ntt(&mut self) {
        for i in 0..self.params.k {
            self.polynomials[i].ntt();
        }
    }

    pub fn inv_ntt(&mut self) {
        for i in 0..self.params.k {
            self.polynomials[i].inv_ntt();
        }
    }
}
    
