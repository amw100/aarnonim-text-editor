mod editor;
use editor::Editor;

fn main() {
    let editor: Editor = Editor::default();
    editor.run();
}