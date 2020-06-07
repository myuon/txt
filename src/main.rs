use std::error::Error;
use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Paragraph, Text};
use tui::Terminal;

mod editor;
mod event;
mod file_buffer;
mod file_manager;
mod line_buffer;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new(Key::Char('q'));

    let mut editor = editor::Editor::new();
    editor.load_file("src/main.rs")?;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Length(f.size().height - 2),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let text = editor
                .get_text()
                .into_iter()
                .map(|t| Text::raw(t))
                .collect::<Vec<_>>();
            let paragraph = Paragraph::new(text.iter()).wrap(true);
            f.render_widget(paragraph, chunks[0]);
            editor.set_editor_size(chunks[0].width, chunks[0].height);

            let text = [Text::raw("= TXT EDIT =")];
            let paragraph = Paragraph::new(text.iter())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .wrap(true);
            f.render_widget(paragraph, chunks[1]);

            f.render_widget(Paragraph::new([Text::raw("")].iter()), chunks[2]);
        })?;

        let cursor = editor.get_cursor();
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(cursor.x + 1, cursor.y + 1)
        )?;

        while let event::Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => return Ok(()),
                Key::Left => editor.cursor_left(),
                Key::Right => editor.cursor_right(),
                Key::Up => editor.cursor_up(),
                Key::Down => editor.cursor_down()?,
                _ => (),
            }
        }
    }

    Ok(())
}
