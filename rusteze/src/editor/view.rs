use super::terminal::{Size, Terminal};
use std::io::Error;
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer
}

impl View {

    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::get_size()?;

        for cur in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(cur) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            
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