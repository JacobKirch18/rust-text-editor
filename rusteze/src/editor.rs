use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::Terminal;

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

    fn refresh_shell(&self) -> Result<(), std::io::Error> {
        if self.exit_token {
            Terminal::clear_shell()?;
            println!("Kachow.\r");
        } else {
            Self::draw_tildes()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
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

    fn draw_tildes() -> Result<(), std::io::Error> {
        let height = Terminal::get_size()?.1;
        for cur in 0..height {
            print!("~");
            if cur + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}