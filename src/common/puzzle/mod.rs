//! Modules for handling puzzle selection and input.

pub use self::selector::PuzzleSelection;
pub use self::solution::{Solution, Answer};

pub mod input;
mod selector;
mod solution;
