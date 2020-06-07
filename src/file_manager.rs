use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};

struct FlexReader<T> {
    reader: T,
}

impl<T: Seek + BufRead> FlexReader<T> {
    pub fn from_reader(reader: T) -> Self {
        FlexReader { reader }
    }

    pub fn read_line(&mut self) -> Result<(String, usize), Box<dyn Error>> {
        let mut s = String::new();
        let u = self.reader.read_line(&mut s)?;

        Ok((s, u))
    }

    pub fn read_lines(&mut self, limit: u32) -> Result<Vec<String>, Box<dyn Error>> {
        let mut ss = vec![];

        for _ in 0..limit {
            let (s, u) = self.read_line()?;

            // EOF
            if u == 0 {
                break;
            }

            ss.push(s);
        }

        Ok(ss)
    }

    pub fn read_line_backward(&mut self) -> Result<(String, usize), Box<dyn Error>> {
        let mut s = String::new();
        let mut seek_position = self.reader.seek(SeekFrom::Current(0))? as i64;
        let chunk_size: u8 = 200;

        seek_position = (seek_position - (chunk_size as i64)).max(0);
        self.reader.seek(SeekFrom::Start(seek_position as u64))?;

        loop {
            let mut buf = vec![];
            self.reader.read_until(chunk_size, &mut buf)?;

            let t = String::from_utf8(buf)?;
            if t.contains("\n") {
                let mut newline_flag = false;
                let mut t_splitted = t.split("\n").collect::<Vec<_>>();
                if t_splitted[t_splitted.len() - 1] == "" {
                    t_splitted.pop();
                    newline_flag = true;
                }

                s = t_splitted[t_splitted.len() - 1].to_string()
                    + if newline_flag { "\n" } else { "" }
                    + s.as_str();
                break;
            } else {
                s = t + s.as_str();
            }

            seek_position = (seek_position - (chunk_size as i64)).max(0);
            if seek_position == 0 {
                break;
            }

            self.reader.seek(SeekFrom::Start(seek_position as u64))?;
        }

        let len = s.len();
        self.reader.seek(SeekFrom::Current(-(len as i64)))?;

        Ok((s, len))
    }

    pub fn read_lines_backward(&mut self, limit: i32) -> Result<Vec<String>, Box<dyn Error>> {
        let mut ss = vec![];

        for _ in 0..limit {
            let (s, u) = self.read_line_backward()?;

            // BOF
            if u == 0 {
                break;
            }

            ss.push(s);
        }

        Ok(ss)
    }

    pub fn seek_to_top(&mut self) -> Result<(), Box<dyn Error>> {
        self.reader.seek(SeekFrom::Start(0))?;

        Ok(())
    }

    pub fn seek_to_end(&mut self) -> Result<(), Box<dyn Error>> {
        self.reader.seek(SeekFrom::End(0))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_line() {
        let reader = Cursor::new("aaaaaa\nbbbbb\nccc\ndd");
        let mut r = FlexReader::from_reader(reader);

        assert_eq!(r.read_line().unwrap().0, "aaaaaa\n");
        assert_eq!(r.read_line().unwrap().0, "bbbbb\n");
        assert_eq!(r.read_line().unwrap().0, "ccc\n");
        assert_eq!(r.read_line().unwrap().0, "dd");
    }

    #[test]
    fn read_lines() {
        let cases = vec![
            ("aaaaaa\nbbbbb\nccc\ndd", 2, vec!["aaaaaa\n", "bbbbb\n"]),
            (
                "aaaaaa\nbbbbb\nccc\ndd",
                5,
                vec!["aaaaaa\n", "bbbbb\n", "ccc\n", "dd"],
            ),
            ("aaaaaa", 2, vec!["aaaaaa"]),
            ("", 1, vec![]),
        ];

        for (s, i, r) in cases {
            let reader = Cursor::new(s);
            let mut reader = FlexReader::from_reader(reader);

            assert_eq!(reader.read_lines(i).unwrap(), r);
        }
    }

    #[test]
    fn read_line_backward_from_end() {
        let cases = vec![
            ("", ""),
            ("aaaaaa", "aaaaaa"),
            ("aaaaaa\nbbbbb\nccc\ndd", "dd"),
            ("aaaa\nbbbb\nccc\n", "ccc\n"),
        ];

        for (s, r) in cases {
            let reader = Cursor::new(s);
            let mut reader = FlexReader::from_reader(reader);
            reader.seek_to_end().unwrap();

            assert_eq!(reader.read_line_backward().unwrap().0, r);
        }
    }

    #[test]
    fn read_lines_backward() {
        let cases = vec![
            ("", 2, vec![]),
            ("aaaaaa", 1, vec!["aaaaaa"]),
            (
                "aaaaaa\nbbbbb\nccc\ndd",
                4,
                vec!["aaaaaa\n", "bbbbb\n", "ccc\n", "dd"],
            ),
        ];

        for (s, i, r) in cases {
            let reader = Cursor::new(s);
            let mut reader = FlexReader::from_reader(reader);
            reader.seek_to_end().unwrap();

            assert_eq!(reader.read_lines_backward(i).unwrap(), r);
        }
    }
}

pub struct FileManager {
    current_file: Option<FlexReader<BufReader<File>>>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager { current_file: None }
    }

    pub fn open(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open(path)?);
        self.current_file = Some(FlexReader::from_reader(reader));

        Ok(())
    }

    pub fn read_n_lines(&mut self, lines: usize) -> Result<Vec<String>, Box<dyn Error>> {
        let mut vs = Vec::new();
        for _ in 0..lines {
            let (line, _) = self.current_file.as_mut().unwrap().read_line()?;

            vs.push(line);
        }

        Ok(vs
            .into_iter()
            .map(|s| s.replace(" ", "\u{2800}"))
            .collect::<Vec<_>>())
    }
}
