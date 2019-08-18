//! Modules for handling puzzle selection and input.

use std::error::Error;
use std::result;

// Module reexports
pub use self::error::SelectionError;
pub use self::input::{fetch_lines, fetch_string};
pub use self::selector::Selection;
pub use self::solution::{Answer, Solution};
pub use self::summary::Summary;

/// Specialized result type for puzzle processing.
pub type Result = result::Result<Solution, Box<dyn Error + Send + Sync + 'static>>;

mod error;
pub mod input;
mod selector;
mod solution;
mod summary;
