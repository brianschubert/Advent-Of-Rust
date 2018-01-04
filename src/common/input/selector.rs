use std::env;

// Range of years with solutions
const MIN_YEAR: Year = 2016;
const MAX_YEAR: Year = 2016;

// Range of valid days
const MIN_DAY: Day = 0;
const MAX_DAY: Day = 25;

type Year = u16;

type Day = u8;

#[derive(Debug, Copy, Clone)]
/// Structure identifying a distinct puzzle solution by year and day.
pub struct PuzzleSelection {
    year: Year,
    day: Day
}

impl PuzzleSelection {

    /// Builds a `PuzzleSelection` for the specified year and day.
    pub fn of(year: Year, day: Day) -> PuzzleSelection {
        PuzzleSelection { year, day }
    }

    /// Attempts to parse a puzzle selection from the specified command-line
    /// arguments.
    pub fn new(mut args: env::Args) -> Result<PuzzleSelection, String> {
        // Ignore executable path
        args.next();

        // Parse the puzzle's year
        let year = match args.next() {
            Some(ys) => match ys.parse::<Year>() {
                Ok(y @ MIN_YEAR ... MAX_YEAR) => y,
                _ => return Err(
                    format!("year must be an integer in the range [{},{}].",
                            MIN_YEAR,
                            MAX_YEAR,
                    )
                )
            },
            None => return Err("year not specified".to_owned())
        };

        // Parse the puzzle's day
        let day = match args.next() {
            Some(ds) => match ds.parse::<Day>() {
                Ok(d @ MIN_DAY ... MAX_DAY) => d,
                _ => return Err(
                    format!("day must be an integer in the range [{},{}].",
                            MIN_DAY,
                            MAX_DAY,
                    )
                )
            },
            None => return Err("day not specified".to_owned())
        };

        Ok(PuzzleSelection { year, day })
    }

    /// Returns the year associated with this puzzle selection.
    pub fn year(&self) -> Year {
        self.year
    }

    /// Returns the day associated with this puzzle selection.
    pub fn day(&self) -> Day {
        self.day
    }

    /// Returns the path to the input file associated with this puzzle selection.
    ///
    /// Visibility limited to the input module.
    pub(super) fn path(&self) -> String {
        format!("./resources/y{:4}/day{:02}.txt", self.year, self.day).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_selection() {
        let pz = PuzzleSelection::of(2017, 18);
        assert_eq!(18, pz.day());
        assert_eq!(2017, pz.year());
    }

    #[test]
    fn path_builds_correctly() {
        assert_eq!(
            "./resources/y2016/day11.txt",
            PuzzleSelection::of(2016, 11).path()
        )
    }

    #[test]
    fn path_padded_with_zeros(){
        assert_eq!(
            "./resources/y2016/day01.txt",
            PuzzleSelection::of(2016, 1).path()
        )
    }
}
