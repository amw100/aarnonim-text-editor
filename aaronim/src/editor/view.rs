use std::io::Error;

use crate::editor::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {


    pub fn render_welcome_screen() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        #[allow(clippy::integer_division)]
        let welcome_row = height / 3;
        for row in 0..height {
            if row == welcome_row {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                Terminal::move_caret_to(Position {x: 0, y: row.saturating_add(1)})?;
            }
        }
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                Terminal::move_caret_to(Position {x: 0, y: row.saturating_add(1)})?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }

    fn draw_welcome_message() -> Result<(), Error> {
        Terminal::clear_line()?;
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

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::clear_line()?;
        Terminal::print("~")?;
        Ok(())
    }
}
