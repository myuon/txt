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

mod event;
mod file_manager;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new(Key::Char('q'));

    let mut filer = file_manager::FileManager::new();
    filer.open("src/main.rs")?;

    let loaded = filer
        .read_n_lines(20)?
        .into_iter()
        .map(|s| s.replace(" ", "\u{2800}"))
        .collect::<Vec<_>>();

    let mut pos = (1, 1);

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

            let text = loaded.iter().map(|v| Text::raw(v)).collect::<Vec<_>>();
            let paragraph = Paragraph::new(text.iter()).wrap(true);
            f.render_widget(paragraph, chunks[0]);

            let text = [Text::raw("= TXT EDIT =")];
            let paragraph = Paragraph::new(text.iter())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .wrap(true);
            f.render_widget(paragraph, chunks[1]);

            f.render_widget(Paragraph::new([Text::raw("")].iter()), chunks[2]);
        })?;

        write!(terminal.backend_mut(), "{}", Goto(pos.0, pos.1))?;

        if let event::Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => break,
                Key::Up if pos.1 > 1 => pos.1 -= 1,
                Key::Down => pos.1 += 1,
                Key::Left if pos.0 > 1 => pos.0 -= 1,
                Key::Right => pos.0 += 1,
                _ => (),
            }
        }
    }

    Ok(())
}
