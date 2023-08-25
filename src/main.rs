mod editor;
mod terminal;
mod document;
mod row;


use editor::Editor;
pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}