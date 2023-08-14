use crate::{params::*, poly::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub pointer: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            data: Vec::new(),
            pointer: 0,
        }
    }

    pub fn zero_initialise(n: usize) -> Self {
        Buffer {
            data: vec![0; n],
            pointer: 0,
        }
    }

    // Write to our bytes buffer
    pub fn push(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    // Read `length` bytes from the buffer starting from current pointer position
    // If there are enough bytes to read, returns a reference to the read slice of bytes
    // If there are not enough bytes, panics
    pub fn read(&mut self, length: usize) -> &[u8] {
        if self.pointer + length <= self.data.len() {
            let slice = &self.data[self.pointer..self.pointer + length];
            self.pointer += length;
            return slice;
        } else {
            panic!("Not enough bytes to read");
        }
    }

    // Set the pointer back to 0
    pub fn reset(&mut self) {
        self.pointer = 0;
    }

    // Packs given poly into a 384-byte buffer
    pub fn pack(&mut self, poly: Poly) {
        for i in 0..N / 2 {
            let t0 = poly.coeffs[2 * i];
            let t1 = poly.coeffs[2 * i + 1];

            self.data[3 * i] = t0 as u8;
            self.data[3 * i + 1] = ((t0 >> 8) | (t1 << 4)) as u8;
            self.data[3 * i + 2] = (t1 >> 4) as u8;
        }
    }

    // Convert a given polynomial into a 32-byte message
    pub fn msg_from_poly(&mut self, poly: Poly) {
        const Q_16: i16 = Q as i16;
        for i in 0..N / 8 {
            self.data[i] = 0;
            for j in 0..8 {
                let mut x = poly.coeffs[8 * i + j];
                x += (x >> 15) & Q_16;
                x = (((x << 1) + Q_16 / 2) / Q_16) & 1;
                self.data[i] |= (x << j) as u8;
            }
        }
    }
}