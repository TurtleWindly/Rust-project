#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod viewer;

use crate::editor::Editor;
fn main() {
    let mut editor = Editor::new();
    editor.run();
}

