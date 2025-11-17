use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Size, Position};

pub struct Editor {
    exit_token: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {exit_token: false}
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_shell(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.exit_token {
            Terminal::clear_shell()?;
            Terminal::print("Kachow.\r\n")?;
        } else {
            Self::draw_tildes()?;
            Terminal::move_cursor_to(Position{x:0, y:0})?;
        }
        Terminal::show_cursor()?;
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
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.exit_token = true;
                }
                _ => (),
            }
        }
    }

    fn draw_tildes() -> Result<(), Error> {
        let Size{height, ..} = Terminal::get_size()?;
        for cur in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if cur + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}