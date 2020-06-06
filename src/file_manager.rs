use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct FileManager {
    current_file: Option<BufReader<File>>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager { current_file: None }
    }

    pub fn open(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open(path)?);
        self.current_file = Some(reader);

        Ok(())
    }

    pub fn read_n_lines(&mut self, lines: usize) -> Result<Vec<String>, Box<dyn Error>> {
        let mut vs = Vec::new();
        for _ in 0..lines {
            let mut line = String::new();
            self.current_file.as_mut().unwrap().read_line(&mut line)?;

            vs.push(line);
        }

        Ok(vs
            .into_iter()
            .map(|s| s.replace(" ", "\u{2800}"))
            .collect::<Vec<_>>())
    }
}
