use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};
use std::{cmp::min, io::Error};

mod terminal;
use terminal::Terminal;
mod view;
use view::View;

use crate::editor::terminal::{Position, Size};

#[derive(Clone, Copy, Default)]
pub struct Location {
    row: usize,
    column: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load_file(filename);
        }
    }
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_location(&mut self, code: KeyCode) -> Result<(), Error> {
        let Location {
            mut row,
            mut column,
        } = self.location;
        let Size { height, width } = Terminal::size()?;
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
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_location(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("GOODBYE NUTS!\r\n")?;
        } else {
            Terminal::move_caret_to(Position::default())?;
            self.view.render()?;
            Terminal::move_caret_to(Position {
                x: self.location.column,
                y: self.location.row,
            })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }
}
