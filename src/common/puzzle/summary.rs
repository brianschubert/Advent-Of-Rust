//! Capture the results from running a puzzle solution.

use std::fmt;
use std::time::Duration;
use super::PuzzleResult;
use super::selector::PuzzleSelection;
use super::error::SelectionError;

/// Text preceding selection error message.
const SELECTION_ERROR_START: &'static str = "Error selecting puzzle";

/// Text preceding solution error message.
const SOLUTION_ERROR_START: &'static str = "Failed to execute solution";

#[derive(Debug)]
/// A summary of the execution of a puzzle's solution.
pub struct Summary {
    puzzle: PuzzleSelection,
    result: PuzzleResult,
    duration: Duration,
}

impl Summary {
    /// Builds a new `Summary` with the specified puzzle selection,
    /// solution result, and execution duration.
    pub fn new(
        puzzle: PuzzleSelection,
        result: PuzzleResult,
        duration: Duration,
    ) -> Self {
        Summary { puzzle, result, duration }
    }

    /// Returns the time spent during puzzle setup.
    ///
    /// The time spent routing to the desired puzzle solution and
    /// reading/parsing the puzzle's input.
    ///
    /// Determined by subtracting each parts execution time from the
    /// total duration.
    pub fn setup_time(&self) -> Duration {
        self.duration - match self.result {
            Ok(ref solution) => solution.duration(),
            Err(..) => Duration::default(),
        }
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.result {
            Err(ref e) => match e.downcast_ref::<SelectionError>() {
                Some(pe) => writeln!(f, "{}: {}", SELECTION_ERROR_START, pe),
                None => writeln!(f, "{}: {}", SOLUTION_ERROR_START, e),
            },
            Ok(ref solution) => {
                let setup_bench = self.setup_time();
                writeln!(f, "Input: {}\n", self.puzzle.path_str())?;
                writeln!(f, "{}", solution)?;
                writeln!(f, "Setup, Parsing: {}.{:09}s", setup_bench.as_secs(), setup_bench.subsec_nanos())?;
                writeln!(f, "Total Elapsed: {}.{:09}s", self.duration.as_secs(), self.duration.subsec_nanos())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use super::super::solution::{Solution, Answer};

    #[test]
    fn displays_error_on_selection_failure() {
        let error = Box::new(SelectionError::UnimplementedDay);

        let summary = Summary::new(
            PuzzleSelection::new(2016, 1),
            Result::Err(error.clone()),
            Duration::default(),
        );

        let out = format!("{}", &summary);
        assert!(out.starts_with(SELECTION_ERROR_START));
        assert!(out.trim_end().ends_with(error.description()));
    }

    #[test]
    fn displays_error_on_solution_failure() {
        let err_msg = "some error";
        let error: Box<dyn Error + Send + Sync> = From::from(err_msg);

        let summary = Summary::new(
            PuzzleSelection::new(2016, 1),
            Result::Err(error),
            Duration::default(),
        );

        let out = format!("{}", &summary);
        assert!(out.starts_with(SOLUTION_ERROR_START));
        assert!(out.trim_end().ends_with(err_msg));
    }

    #[test]
    fn determine_setup_time() {
        let summary = Summary::new(
            PuzzleSelection::new(2016, 1),
            Ok(Solution::new(
                Some(Answer::with_bench("one", Some(Duration::new(10, 100)))),
                Some(Answer::with_bench("two", Some(Duration::new(5, 20)))),
            )),
            Duration::new(100, 1000),
        );

        assert_eq!(
            Duration::new(85, 880),
            summary.setup_time()
        )
    }
}