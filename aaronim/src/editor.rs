use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};
use std::{
    cmp::min,
    io::Error,
    panic::{set_hook, take_hook},
};

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
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        let mut view = View::default();
        Terminal::initialize()?;
        Self::handle_args(&mut view);
        Ok(Self {
            should_quit: false,
            location: Location { row: 0, column: 0 },
            view,
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                #[allow(unused_variables)]
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("Could not read event: {err:?}");
                }
            }
        }
    }

    fn handle_args(view: &mut View) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            view.load_file(filename);
        }
    }

    fn move_location(&mut self, code: KeyCode) {
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

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_location(code);
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size { height, width });
            }
            _ => {}
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            x: self.location.column,
            y: self.location.row,
        });
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("GOODBYE NUTS!\r\n");
            let _ = Terminal::execute();
        }
    }
}
