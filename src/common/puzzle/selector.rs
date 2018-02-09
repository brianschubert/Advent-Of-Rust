//! Puzzle solution selection.

use std::env;
use std::path::Path;
use super::error::SelectionError;

/// Underlying type representing a puzzle's year.
type Year = u16;

/// Underlying type representing a puzzle's day.
type Day = u8;

#[derive(Debug)]
/// Structure identifying a distinct puzzle solution by year and day.
pub struct PuzzleSelection {
    year: Year,
    day: Day,
    input: String, // the path, not the puzzle input
}

impl PuzzleSelection {
    /// Builds a `PuzzleSelection` for the specified year and day.
    pub fn new(year: Year, day: Day) -> Self {
        let input = default_input_for(year, day);
        PuzzleSelection::with_input(year, day, input)
    }

    /// Builds a `PuzzleSelection` for the specified year and day with
    /// the specified input file.
    pub fn with_input<P>(year: Year, day: Day, input_file: P) -> Self
        where P: ToString
    {
        PuzzleSelection { year, day, input: input_file.to_string() }
    }

    /// Attempts to parse a puzzle selection from the specified command-line
    /// arguments.
    pub fn from_args(mut args: env::Args) -> Result<PuzzleSelection, SelectionError> {
        // Ignore executable path
        args.next();

        let year: Year = args.next()
            .ok_or(SelectionError::NoSelection)?
            .parse()
            .map_err(|_| SelectionError::BadYear)?;

        let day: Day = args.next()
            .ok_or(SelectionError::NoSelection)?
            .parse()
            .map_err(|_| SelectionError::BadDay)?;

        let input = args.next()
            .unwrap_or_else(|| default_input_for(year, day));

        Ok(PuzzleSelection::with_input(year, day, input))
    }

    /// Returns the year associated with this puzzle selection.
    pub fn year(&self) -> Year {
        self.year
    }

    /// Returns the day associated with this puzzle selection.
    pub fn day(&self) -> Day {
        self.day
    }

    /// Returns the path to the input file associated with this puzzle
    /// selection.
    pub fn path(&self) -> &Path {
        Path::new(&self.input)
    }

    /// Returns the input file path as a string slice.
    pub fn path_str(&self) -> &str {
        &self.input[..]
    }
}

impl AsRef<PuzzleSelection> for PuzzleSelection {
    fn as_ref(&self) -> &Self { self }
}

/// Returns the default path for a puzzle's input file.
fn default_input_for(year: Year, day: Day) -> String {
    format!("./resources/y{:4}/day{:02}.txt", year, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_selection() {
        let pz = PuzzleSelection::new(2017, 5);
        assert_eq!(5, pz.day());
        assert_eq!(2017, pz.year());
    }

    #[test]
    fn path_builds_correctly() {
        assert_eq!(
            "./resources/y2016/day11.txt",
            PuzzleSelection::new(2016, 11).path_str()
        )
    }

    #[test]
    fn path_padded_with_zeros() {
        assert_eq!(
            "./resources/y2016/day01.txt",
            PuzzleSelection::new(2016, 1).path_str()
        )
    }

    #[test]
    fn override_input_path() {
        assert_eq!(
            "./resources/y2016/day02.txt",
            PuzzleSelection::with_input(2016, 1, "./resources/y2016/day02.txt").path_str()
        )
    }
}
