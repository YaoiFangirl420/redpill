extern crate tui;
extern crate termion;

use std::io;
use std::io::stdin;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};
use termion::color;
use termion::clear;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};

type TerminalTarget = tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>;

fn draw_main_ui(terminal: &mut TerminalTarget, words: Vec<String>) {

  let title = format!("redpill version {}", env!("CARGO_PKG_VERSION"));

  terminal.draw(|mut f| {

    let size = f.size();

    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints(
        [
        Constraint::Percentage(90),
        Constraint::Percentage(10),
        ].as_ref()
        )
      .split(size);

    let block = Block::default()
      .title(&title)
      .borders(Borders::ALL);

    let mut text = vec![];
    for word in words.iter() {
      text.push(Text::raw(format!("{}\n", word)));
    }

    Paragraph::new(text.iter())
      .block(block)
      .wrap(true)
      .render(&mut f, chunks[0]);

    Block::default()
      .title("Input")
      .borders(Borders::ALL)
      .render(&mut f, chunks[1]);
  });
}

fn main() -> Result<(), io::Error> {
  /* initialize screen */
  println!("{}", clear::All);
  let stdin = stdin();
  let stdout = io::stdout().into_raw_mode()?;
  let backend: TermionBackend<termion::raw::RawTerminal<std::io::Stdout>> = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let (keys_write, keys_read) = mpsc::channel();

  let stdin_thread = thread::spawn(move || {
    for c in stdin.events() {
      let evt = c.unwrap();
      match evt {
        Event::Key(key) => keys_write.send(key).unwrap(),
        _ => (),
      }
    }
  });


  let mut input_buf = String::new();
  draw_main_ui(&mut terminal, vec![format!("aye")]);
  loop {
    if let Ok(key) = keys_read.try_recv() {
      match key {
        Key::Ctrl('q') => break,
        Key::Char('\n') => {
          input_buf.clear();
          draw_main_ui(&mut terminal, vec![format!("aye"), format!("{}", input_buf)]);
        },
        Key::Char(c) => {
          input_buf.push(c);
          draw_main_ui(&mut terminal, vec![format!("aye"), format!("{}", input_buf)]);
        },
        _ => (),
      }
    }
  }

  Ok(())
}
