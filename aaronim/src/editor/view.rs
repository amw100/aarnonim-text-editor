use std::cmp::min;

use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;
use crossterm::event::KeyCode;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
pub struct Location {
    row: usize,
    column: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location { row: 0, column: 0 },
        }
    }
}

impl View {
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) {
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        if self.needs_redraw {
            if self.buffer.is_empty() {
                self.render_welcome_screen();
            } else {
                self.render_buffer();
            }
            self.needs_redraw = false;
        }
        let _ = Terminal::move_caret_to(Position {
            x: self.location.column,
            y: self.location.row,
        });
    }

    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn move_location(&mut self, code: KeyCode) {
        let Location {
            mut row,
            mut column,
        } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match code {
            KeyCode::Up => {
                row = row.saturating_sub(1);
            }
            KeyCode::Down => {
                row = min(height.saturating_sub(1), row.saturating_add(1));
            }
            KeyCode::Right => {
                column = min(width.saturating_sub(1), column.saturating_add(1));
            }
            KeyCode::Left => {
                column = column.saturating_sub(1);
            }
            KeyCode::PageUp => {
                row = 0;
            }
            KeyCode::PageDown => {
                row = height.saturating_sub(1);
            }
            KeyCode::End => {
                column = width.saturating_sub(1);
            }
            KeyCode::Home => {
                column = 0;
            }
            _ => (),
        }
        self.location = Location { row, column };
    }

    fn render_welcome_screen(&self) {
        let Size { height, .. } = self.size;
        #[allow(clippy::integer_division)]
        let welcome_row = height / 3;
        for row in 0..height {
            if row == welcome_row {
                self.draw_welcome_message(row);
            } else {
                Self::render_line(row, "~");
            }
        }
    }

    fn render_buffer(&self) {
        let Size { height, width } = self.size;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let mut line_to_print = String::from(line);
                line_to_print.truncate(width);
                Self::render_line(row, &line_to_print);
            } else {
                Self::render_line(row, "~");
            }
        }
    }

    fn render_line(row_at: usize, line_contents: &str) {
        let result = Terminal::print_line(row_at, line_contents);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn draw_welcome_message(&self, row: usize) {
        let width = self.size.width;
        let mut message = format!("{NAME} NUTS EDITOR -- version {VERSION}");
        let len = message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{spaces}{message}");
        message.truncate(width);
        Self::render_line(row, &message);
    }
}
