use crate::poly::*;

#[derive(Clone, PartialEq)]
pub struct PolyVec {
    pub polynomials: Vec<Poly>, // Vec of K polynomials, where K is the security level
}

impl PolyVec {
    // Adds the given vector of polynomial and sets self to be the sum
    // Example:
    // vec1.add(vec2);
    pub fn add(&mut self, x: &PolyVec) {
        assert_eq!(self.polynomials.len(), x.polynomials.len());
        for i in 0..self.polynomials.len() {
            self.polynomials[i].add(&x.polynomials[i]);
        }
    }

    pub fn reduce(&mut self) {
        for i in 0..self.polynomials.len() {
            self.polynomials[i].reduce();
        }
    }

    pub fn normalise(&mut self) {
        for i in 0..self.polynomials.len() {
            self.polynomials[i].reduce();
        }
    }

    pub fn ntt(&mut self) {
        for i in 0..self.polynomials.len() {
            self.polynomials[i].ntt();
        }
    }

    pub fn inv_ntt(&mut self) {
        for i in 0..self.polynomials.len() {
            self.polynomials[i].inv_ntt();
        }
    }
}
