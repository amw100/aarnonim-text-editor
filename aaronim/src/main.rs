#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(filename) = args.get(1) {
        Editor::new(filename.clone()).run();
    } else {
        Editor::default().run();
    }              
}
