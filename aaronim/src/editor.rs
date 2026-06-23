use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        Self::draw_rows()?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("GOODBYE NUTS!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(terminal::Position { x: 0, y: 0 })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let terminal::Size {height, ..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
