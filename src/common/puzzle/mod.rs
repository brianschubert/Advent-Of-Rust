//! Modules for handling puzzle selection and input.

use std::error::Error;

// Module reexports
pub use self::error::SelectionError;
pub use self::selector::PuzzleSelection;
pub use self::solution::{Solution, Answer};
pub use self::summary::Summary;

/// Specialized result type for puzzle processing.
pub type PuzzleResult = Result<Solution, Box<dyn Error + Send + Sync + 'static>>;

pub mod input;
mod error;
mod selector;
mod solution;
mod summary;
