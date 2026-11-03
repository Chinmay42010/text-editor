mod editor;

use editor::editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
