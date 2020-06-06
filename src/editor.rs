use crate::file_manager::FileManager;
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
    text: Vec<String>,
    width: u16,
    height: u16,
    file_manager: FileManager,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor: Cursor { x: 0, y: 0 },
            text: Vec::new(),
            width: 0,
            height: 0,
            file_manager: FileManager::new(),
        }
    }

    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.file_manager.open(path)?;
        self.cursor.reset();
        self.text = self.file_manager.read_n_lines(self.height as usize)?;

        Ok(())
    }

    fn load_next_line(&mut self) -> Result<(), Box<dyn Error>> {
        let mut lines = self.file_manager.read_n_lines(1)?;
        self.text.remove(0);
        self.text.push(lines.pop().unwrap());

        Ok(())
    }

    pub fn get_text_ref(&self) -> &Vec<String> {
        &self.text
    }

    pub fn set_editor_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn get_cursor(&self) -> Cursor {
        let cursor = self.cursor.clone();

        Cursor {
            x: if self.text.len() == 0 {
                0
            } else {
                (cursor.x).min(self.text[self.cursor.y as usize].len() as u16 - 1)
            },
            y: cursor.y,
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub fn cursor_down(&mut self) -> Result<(), Box<dyn Error>> {
        if self.cursor.y < (self.height).min(self.text.len() as u16) - 1 {
            self.cursor.y += 1;
        } else if self.cursor.y == self.text.len() as u16 - 1 {
            self.load_next_line()?;
        }

        Ok(())
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x == 0 {
            self.cursor_up();
            self.cursor.x = self.text[self.cursor.y as usize].len() as u16 - 1;
        } else if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        let current_length = {
            let current_line = self.text[self.cursor.y as usize]
                .chars()
                .collect::<Vec<_>>();

            if current_line[current_line.len() - 1] == '\n' {
                current_line.len() - 1
            } else {
                current_line.len()
            }
        } as u16;

        if self.cursor.x == (self.width).min(current_length) {
            self.cursor.x = 0;
            self.cursor_down();
        } else if self.cursor.x < (self.width).min(current_length) {
            self.cursor.x += 1;
        }
    }
}
