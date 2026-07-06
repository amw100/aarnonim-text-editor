use std::io::Error;

use crate::editor::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }

    fn render_welcome_screen(&self) -> Result<(), Error> {
        let Size { height, .. } = self.size;
        #[allow(clippy::integer_division)]
        let welcome_row = height / 3;
        for row in 0..height {
            if row == welcome_row {
                self.draw_welcome_message(row)?;
            } else {
                Self::render_line(row, "~")?;
            }
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, width } = self.size;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let mut line_to_print = String::from(line);
                line_to_print.truncate(width);
                Self::render_line(row, &line_to_print)?;
            } else {
                Self::render_line(row, "~")?;
            }
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        if self.buffer.is_empty() {
            self.render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        self.needs_redraw = false;
        Ok(())
    }

    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    fn render_line(row_at: usize, line_contents: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { x: 0, y: row_at })?;
        Terminal::clear_line()?;
        Terminal::print(line_contents)?;
        Ok(())
    }

    fn draw_welcome_message(&self, row: usize) -> Result<(), Error> {
        let width = self.size.width;
        let mut message = format!("{NAME} NUTS EDITOR -- version {VERSION}");
        let len = message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{spaces}{message}");
        message.truncate(width);
        Self::render_line(row, &message)?;
        Ok(())
    }
}
