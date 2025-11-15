use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    exit_token: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor {exit_token: false}
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Kachow.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.exit_token = true;
                    }
                    _ => (),
                }
            }
            if self.exit_token {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}