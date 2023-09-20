use crate::{params::*, poly::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub pointer: usize,
}

impl Buffer {
    // Creates a new, empty buffer
    // Example:
    // let buf = Buffer::new();
    pub fn new() -> Self {
        Buffer {
            data: Vec::new(),
            pointer: 0,
        }
    }

    // Creates a new buffer of length n, with all elements being 0
    // Example:
    // let buf = Buffer::zero_initialise();
    pub fn zero_initialise(n: usize) -> Self {
        Buffer {
            data: vec![0; n],
            pointer: 0,
        }
    }

    // Write to our bytes buffer
    // Example:
    // buf.push(byte_slice);
    pub fn push(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    // Read `length` bytes from the buffer starting from current pointer position
    // If there are enough bytes to read, returns a reference to the read slice of bytes
    // If there are not enough bytes, panics
    // Example:
    // let bytes = buf.read(5);
    pub fn read(&mut self, length: usize) -> &[u8] {
        if self.pointer + length <= self.data.len() {
            let slice = &self.data[self.pointer..self.pointer + length];
            self.pointer += length;
            slice
        } else {
            panic!("Not enough bytes to read");
        }
    }

    // Set the pointer back to 0
    // Example:
    // buf.reset();
    pub fn reset(&mut self) {
        self.pointer = 0;
    }

    // Packs given poly into a 384-byte buffer
    // Example:
    // buf.pack(poly);
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
    // Example:
    // msg.msg_from_poly(poly);
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

    // Compress polynomial to a buffer
    // Example:
    // buf.compress(poly);
    pub fn compress(&mut self, poly: Poly, compressed_bytes: usize) {
        let mut k = 0usize;
        let mut t = [0u8; 8];

        match compressed_bytes {
            128 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = poly.coeffs[8 * i + j];
                        u += (u >> 15) & Q as i16;
                        t[j] = (((((u as u16) << 4) + Q as u16 / 2) / Q as u16) & 15) as u8;
                    }
                    self.data[k] = t[0] | (t[1] << 4);
                    self.data[k + 1] = t[2] | (t[3] << 4);
                    self.data[k + 2] = t[4] | (t[5] << 4);
                    self.data[k + 3] = t[6] | (t[7] << 4);
                    k += 4;
                }
            }
            160 => {
                for i in 0..N / 8 {
                    for j in 0..8 {
                        let mut u = poly.coeffs[8 * i + j];
                        u += (u >> 15) & Q as i16;
                        t[j] = (((((u as u32) << 5) + Q as u32 / 2) / Q as u32) & 31) as u8;
                    }
                    self.data[k] = t[0] | (t[1] << 5);
                    self.data[k + 1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
                    self.data[k + 2] = (t[3] >> 1) | (t[4] << 4);
                    self.data[k + 3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
                    self.data[k + 4] = (t[6] >> 2) | (t[7] << 3);
                    k += 5;
                }
            }
            _ => panic!("Invalid compressed poly bytes size."),
        }
    }
}
