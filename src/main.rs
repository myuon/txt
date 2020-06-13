use std::env;
use std::error::Error;
use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

mod editor;
mod event;
mod file_buffer;
mod gap_buffer;
mod line_buffer;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let stdout = io::stdout().into_raw_mode()?;
    let mut stdout = AlternateScreen::from(stdout);

    let events = event::Events::new(Key::Esc);

    let mut editor = editor::Editor::new();
    editor.load_file(&args[1])?;

    let (w, h) = termion::terminal_size()?;
    editor.set_editor_size(w, h);

    loop {
        write!(stdout, "{}{}", termion::clear::All, Goto(1, 1))?;
        for (index, text) in editor.get_text().into_iter().enumerate() {
            write!(stdout, "{}", Goto(1, index as u16 + 1))?;
            write!(stdout, "{}", text)?;
        }

        let cursor = editor.get_cursor();
        write!(stdout, "{}", Goto(cursor.x + 1, cursor.y + 1))?;

        stdout.flush()?;

        if events.exit() {
            break;
        }

        while let event::Event::Input(input) = events.next()? {
            match input {
                Key::Left => editor.cursor_left(),
                Key::Right => editor.cursor_right(),
                Key::Up => editor.cursor_up(),
                Key::Down => editor.cursor_down()?,
                Key::Char(ch) => editor.insert_at_cursor(ch),
                Key::Backspace => editor.delete_at_cursor(),
                Key::Ctrl('s') => editor.save_file()?,
                _ => (),
            }
        }
    }

    Ok(())
}
