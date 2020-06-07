pub struct LineBuffer {
    buffer: Vec<char>,
}

impl LineBuffer {
    pub fn new(from: &str) -> Self {
        LineBuffer {
            buffer: from.chars().collect(),
        }
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        self.buffer.insert(index, ch);
    }

    pub fn remove(&mut self, index: usize) {
        self.buffer.remove(index);
    }

    pub fn to_string(&self) -> String {
        self.buffer.iter().collect()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
