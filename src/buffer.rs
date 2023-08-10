#[derive(Copy, Clone)]
pub struct Buffer {
    data: Vec<u8>,
    pointer: usize,
}

pub enum BufferError {
    InsufficientData,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            data: Vec::new(),
            pointer: 0,
        }
    }
    
    // Write to our bytes buffer
    pub fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    // Read `length` bytes from the buffer starting from current pointer position
    // If there are enough bytes to read, returns a reference to the read slice of bytes
    // If there are not enough bytes, returns an error. 
    pub fn read(&mut self, length: usize) -> Result<&[u8], BufferError> {
        if self.pointer + length <= self.data.len() {
            let slice = &self.data[self.pointer..self.pointer + length];
            self.pointer += length;
            Ok(slice)
        } else {
            Err(BufferError::InsufficientData)
        }
    }

    pub fn reset(&mut self) {
        self.pointer = 0;
    }
}
