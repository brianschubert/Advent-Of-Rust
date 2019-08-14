//! Error types for handling puzzles on the naughty list.

use std::{error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Errors that may occur while selecting a puzzle solution.
///
/// Represents errors that occur during puzzle selection/setup/routing
/// /divining/guessing.
pub enum SelectionError {
    /// Insufficient arguments to select a puzzle.
    NoSelection,
    /// No solutions exists of the provided year.
    UnimplementedYear,
    /// No solution exists for the provided day.
    UnimplementedDay,
    /// The puzzle's year could not be parsed
    BadYear,
    /// The puzzle's day could not be parsed
    BadDay,
}

impl SelectionError {
    /// Returns a brief description of the setup error for display.
    pub fn as_str(&self) -> &'static str {
        match *self {
            SelectionError::NoSelection => "insufficient arguments to select a puzzle",
            SelectionError::UnimplementedYear => "no solutions exist for the provided year",
            SelectionError::UnimplementedDay => "no solution exists for the provided day",
            SelectionError::BadYear => "puzzle year could not be parsed",
            SelectionError::BadDay => "puzzle day could not be parsed",
        }
    }
}

impl error::Error for SelectionError {
    fn description(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
