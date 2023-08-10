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
            return slice
        } else {
            panic!("Not enough bytes to read");
        }
    }

    // Set the pointer back to 0
    pub fn reset(&mut self) {
        self.pointer = 0;
    }
}
