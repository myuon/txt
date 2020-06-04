#[derive(Clone)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    cursor: Cursor,
    text: Vec<String>,
    width: u16,
    height: u16,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor: Cursor { x: 0, y: 0 },
            text: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn set_page(&mut self, page: Vec<String>) {
        self.text = page;
    }

    pub fn set_editor_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.y < self.height - 1 {
            self.cursor.y += 1;
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x < self.width - 1 {
            self.cursor.x += 1;
        }
    }
}
