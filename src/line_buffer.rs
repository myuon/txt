use crate::gap_buffer::GapBuffer;

pub struct LineBuffer {
    buffer: GapBuffer<char>,
}

impl LineBuffer {
    pub fn new(s: &str) -> Self {
        LineBuffer {
            buffer: GapBuffer::from(s.chars().collect()),
        }
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        self.buffer.insert(index, ch);
    }

    pub fn remove(&mut self, index: usize) {
        self.buffer.delete(index);
    }

    pub fn to_string(&self) -> String {
        self.buffer.as_vec().iter().collect()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
