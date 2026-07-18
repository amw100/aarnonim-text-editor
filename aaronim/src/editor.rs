use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

mod editor_command;
mod terminal;
mod view;
use editor_command::EditorCommand;
use terminal::Terminal;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
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

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("Could not handle command: {err}");
                }
            }
        } else {
            #[cfg(debug_assertions)]
            panic!("Received and discarded unsopported or non-press event");
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
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
