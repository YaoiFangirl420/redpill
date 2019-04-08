extern crate tui;
extern crate termion;

use std::io;
use std::io::stdin;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};
use termion::clear;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {

    println!("{}", clear::All);

    let stdin = stdin();

    let title = format!("redpill version {}", env!("CARGO_PKG_VERSION"));
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|mut f| {
      let size = f.size();

      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
          [
          Constraint::Percentage(80),
          Constraint::Percentage(10),
          Constraint::Percentage(10),
          ].as_ref()
        )
        .split(size);

      let block = Block::default()
        .title(&title)
        .borders(Borders::ALL);

      let text = [
        Text::raw("Line 1"),
        Text::raw("Line 2\n"),
        Text::raw("haha lol"),

      ];
      Paragraph::new(text.iter())
        .block(block)
        .wrap(true)
        .render(&mut f, chunks[0]);

      Block::default()
        .title("YOLO")
        .borders(Borders::ALL)
        .render(&mut f, chunks[2]);
    }).unwrap();

    for c in stdin.events() {
      let evt = c.unwrap();
      match evt {
        Event::Key(Key::Char('q')) => break,
        _ => (),
      }
    }
    Ok(())
}
