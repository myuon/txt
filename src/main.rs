use std::error::Error;
use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph, Text};
use tui::Terminal;

mod event;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = event::Events::new(Key::Char('q'));

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

            let text = [Text::raw("This is a line \nPiyo piyo\n\nfooo")];
            let paragraph = Paragraph::new(text.iter()).wrap(true);
            f.render_widget(paragraph, chunks[0]);

            let text = [Text::raw("= TXT EDIT =")];
            let paragraph = Paragraph::new(text.iter())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .wrap(true);
            f.render_widget(paragraph, chunks[1]);

            f.render_widget(Paragraph::new([Text::raw("")].iter()), chunks[2]);
        })?;

        if let event::Event::Input(input) = events.next()? {
            if let Key::Char('q') = input {
                break;
            }
        }
    }

    Ok(())
}
