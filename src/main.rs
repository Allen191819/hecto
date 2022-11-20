#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::SearchDirection;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
