use super::terminal::{Size, Terminal};
use std::io::Error;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {

    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::get_size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello, World!\r\n")?;

        for cur in 1..height {
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

    fn output_welcome() -> Result<(), Error> {
        let mut message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::get_size()?.width;
        let length = message.len();

        #[allow(clippy::integer_division)]
        let padding = (width - length) / 2;

        let spaces = " ".repeat(padding - 1);
        message = format!("~{spaces}{message}");
        message.truncate(width);
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

}