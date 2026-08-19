use std::cmp::min;

use crate::editor::{
    editor_command::{Direction, EditorCommand},
    view::line::Line,
};

use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

mod location;
use location::Location;
mod line;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    caret_location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            caret_location: Location { row: 0, column: 0 },
            scroll_offset: Location { row: 0, column: 0 },
        }
    }
}

impl View {
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
    }

    pub fn load_file(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_location(&direction),
            EditorCommand::Quit => {}
        }
    }

    pub fn caret_position(&self) -> Position {
        self.caret_location.subtract(&self.scroll_offset).into()
    }

    fn move_location(&mut self, direction: &Direction) {
        let Location {
            mut row,
            mut column,
        } = self.caret_location;
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                row = row.saturating_sub(1);
            }
            Direction::Down => {
                if row < self.buffer.lines.len() {
                    row = row.saturating_add(1);
                }
            }
            Direction::Right => {
                let length = self.buffer.lines.get(row).map_or(0, Line::len);
                if column < length {
                    column = column.saturating_add(1);
                } else {
                    row = row.saturating_add(1);
                    column = 0;
                }
            }
            Direction::Left => {
                if column > 0 {
                    column = column.saturating_sub(1);
                } else if row > 0 {
                    row = row.saturating_sub(1);
                    column = self.buffer.lines.get(row).map_or(0, Line::len);
                }
            }
            Direction::PageUp => {
                row = row.saturating_sub(height);
            }
            Direction::PageDown => {
                row = row.saturating_add(height).saturating_sub(1);
            }
            Direction::End => {
                column = self.buffer.lines.get(row).map_or(0, Line::len);
            }
            Direction::Home => {
                column = 0;
            }
        }
        column = self
            .buffer
            .lines
            .get(row)
            .map_or(0, |line| min(line.len(), column));
        row = min(row, self.buffer.lines.len());
        self.caret_location = Location { row, column };
        self.scroll_into_view();
    }

    fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.scroll_into_view();
        self.needs_redraw = true;
    }

    fn scroll_into_view(&mut self) {
        let Size { height, width } = self.size;
        let Location { row, column } = self.caret_location;
        let mut redraw = false;

        if row >= height.saturating_add(self.scroll_offset.row) {
            self.scroll_offset.row = row.saturating_sub(height).saturating_add(1);
            redraw = true;
        } else if row < self.scroll_offset.row {
            self.scroll_offset.row = row;
            redraw = true;
        }

        if column >= width.saturating_add(self.scroll_offset.column) {
            self.scroll_offset.column = column.saturating_sub(width).saturating_add(1);
            redraw = true;
        } else if column < self.scroll_offset.column {
            self.scroll_offset.column = column;
            redraw = true;
        }

        self.needs_redraw = self.needs_redraw || redraw;
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
        for screen_row in 0..height {
            if let Some(line) = self
                .buffer
                .lines
                .get(screen_row.saturating_add(self.scroll_offset.row))
            {
                let left = self.scroll_offset.column;
                let right = self.scroll_offset.column.saturating_add(width);
                Self::render_line(screen_row, &line.get(left..right));
            } else {
                Self::render_line(screen_row, "~");
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
