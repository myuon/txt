use crate::line_buffer::LineBuffer;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub struct FileBuffer {
    file_path: String,
    buffer: Vec<LineBuffer>,
}

impl FileBuffer {
    pub fn new() -> Self {
        FileBuffer {
            file_path: String::new(),
            buffer: vec![],
        }
    }

    pub fn open(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut file_buffer = vec![];

        let mut buf = String::new();
        while reader.read_line(&mut buf)? > 0 {
            file_buffer.push(LineBuffer::new(&buf));
        }

        self.file_path = path.to_string();
        self.buffer = file_buffer;

        Ok(())
    }

    pub fn get_strings_between(&self, i: usize, j: usize) -> Vec<String> {
        let mut vs = vec![];

        for u in i..j {
            vs.push(self.buffer[u].to_string());
        }

        vs
    }

    pub fn len_at(&self, i: usize) -> usize {
        self.buffer[i].len()
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut writer = BufWriter::new(File::open(&self.file_path)?);
        for line in &self.buffer {
            writer.write(&line.to_string().into_bytes())?;
        }

        Ok(())
    }
}
