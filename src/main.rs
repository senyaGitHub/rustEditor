#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::shadow_reuse,
)]



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