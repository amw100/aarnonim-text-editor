use std::io::Error;

use crate::editor::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            #[allow(clippy::integer_division)]
            if row == 0 {
                Self::draw_hello_world()?;
            }
            else if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let width = Terminal::size()?.width;
        let mut message = format!("{NAME} NUTS EDITOR -- version {VERSION}");
        let len = message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{spaces}{message}");
        message.truncate(width);
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_hello_world() -> Result<(), Error> {
        let width = Terminal::size()?.width;
        let mut message = String::from("HELLO NUTS");
        message.truncate(width);
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::clear_line()?;
        Terminal::print("~")?;
        Ok(())
    }
}
