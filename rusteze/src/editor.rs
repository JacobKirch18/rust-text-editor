use core::cmp::min;
use crossterm::event::{
    read, 
    Event::{self, Key}, 
    KeyCode, 
    KeyEvent,
    KeyEventKind, 
    KeyModifiers,
};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    exit_token: bool,
    loc: Location,
}

impl Editor {

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_shell(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.exit_token {
            Terminal::clear_shell()?;
            Terminal::print("Kachow.\r\n")?;
        } else {
            Self::draw_tildes()?;
            Terminal::move_caret_to(Position {
                col: self.loc.x, 
                row: self.loc.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_shell()?;
            if self.exit_token {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.loc;
        let Size { height, width } = Terminal::get_size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                if x == 0 && y == 0 {
                    // do nothing (top right corner, statement included for clarity)
                }
                else if x == 0 {
                    x = width.saturating_sub(1);
                    y = y.saturating_sub(1);
                } else {
                    x = x.saturating_sub(1);
                }
            }
            KeyCode::Right => {
                if x == width.saturating_sub(1) && y == height.saturating_sub(1) {
                    // do nothing (bottom right corner, statement including for clarity)
                } else if x == width.saturating_sub(1) {
                    x = 0;
                    y = min(height.saturating_sub(1), y.saturating_add(1));
                } else {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.loc = Location { x, y };
        Ok(()) 
    
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, 
            modifiers, 
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.exit_token = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn output_welcome() -> Result<(), Error> {
        let mut message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::get_size()?.width;
        let length = message.len();

        #[allow(clippy::integer_division)]
        let padding = (width - length) / 2;

        let spaces = " ".repeat(padding - 1);
        message = format!("~{spaces}{message}");
        message.truncate(width);
        Terminal::print(message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_tildes() -> Result<(), Error> {
        let Size{height, ..} = Terminal::get_size()?;
        for cur in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if cur == height / 3 {
                Self::output_welcome()?;
            } else {
                Self::draw_empty_row()?;
            }
            if cur.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}