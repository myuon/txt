use crate::file_buffer::FileBuffer;
use std::error::Error;

#[derive(Clone)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Cursor {
    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

pub struct Editor {
    cursor: Cursor,
    width: u16,
    height: u16,
    file_buffer: FileBuffer,
    index_start: usize,
    index_end: usize,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor: Cursor { x: 0, y: 0 },
            width: 0,
            height: 0,
            file_buffer: FileBuffer::new(),
            index_start: 0,
            index_end: 0,
        }
    }

    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.file_buffer.open(path)?;
        self.cursor.reset();

        Ok(())
    }

    pub fn get_text(&self) -> Vec<String> {
        self.file_buffer
            .get_strings_between(self.index_start, self.index_end)
            .into_iter()
            // 0-width character
            .map(|str| str.replace(" ", "\u{2800}"))
            .collect()
    }

    pub fn set_editor_size(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;

            self.index_start = 0;
            self.index_end = height as usize;
        }
    }

    pub fn get_cursor(&self) -> Cursor {
        let cursor = self.cursor.clone();

        Cursor {
            x: (cursor.x).min(self.file_buffer.len_at(cursor.y as usize) as u16 - 1),
            y: cursor.y,
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.y == 0 && self.index_start > 0 {
            self.index_start -= 1;
            self.index_end -= 1;
        }

        if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub fn cursor_down(&mut self) -> Result<(), Box<dyn Error>> {
        if self.cursor.y == self.height - 1 && self.index_end < self.file_buffer.len() {
            self.index_start += 1;
            self.index_end += 1;
        }

        if self.cursor.y < self.height - 1 {
            self.cursor.y += 1;
        }

        Ok(())
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        self.cursor.x += 1;
    }
}
