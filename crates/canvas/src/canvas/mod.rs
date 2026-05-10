mod clipboard;
pub mod document;
mod edit;
mod selection_ops;
pub mod state;

#[cfg(test)]
mod tests;

pub use document::{CanvasDocument, ShapeMut};
pub use selection_ops::AlignMode;
pub use state::Canvas;
