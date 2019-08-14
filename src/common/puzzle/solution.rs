//! Puzzle solution representation

use std::string::ToString;
use std::time::Duration;
use std::fmt;

#[derive(Debug, Eq)]
/// A solution to one part of a day's puzzle with an optional execution time.
pub struct Answer {
    ans: String,
    bench: Option<Duration>,
}

#[derive(Debug, Eq, PartialEq)]
/// A solution to a day's puzzle containing an optional answer for each part.
pub struct Solution(pub Option<Answer>, pub Option<Answer>);

impl Answer {
    /// Builds an answer with the specified value
    pub fn new<S>(other: S) -> Self
        where S: ToString
    {
        Answer { ans: other.to_string(), bench: None }
    }

    /// Builds an answer with the specified value and optional benchmark.
    pub fn with_bench<S>(ans: S, bench: Option<Duration>) -> Self
        where S: ToString
    {
        Answer { ans: ans.to_string(), bench }
    }

    /// Returns a slice to this puzzle part's answer string.
    pub fn ans(&self) -> &str {
        &self.ans[..]
    }

    /// Returns a reference to this puzzle part's optional benchmark.
    pub fn bench(&self) -> Option<&Duration> {
        self.bench.as_ref()
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Answer) -> bool {
        self.ans == other.ans
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` ", self.ans)?;
        match self.bench {
            Some(bench) => write!(
                f, "[{}.{:09}s]",
                bench.as_secs(),
                bench.subsec_nanos()
            ),
            None => write!(f, "[x]")
        }
    }
}

impl Solution {
    /// Builds a solution from the specified optional answers.
    pub fn new(one: Option<Answer>, two: Option<Answer>) -> Solution {
        Solution(one, two)
    }

    /// Builds a solution with `None` for both answers.
    pub fn empty() -> Self {
        Solution(None, None)
    }

    /// Returns the total duration of time elapsed between the two
    /// answers.
    ///
    /// Essentially, the sum of each answers benchmark. Absent
    /// benchmarks are substituted with durations of `0`.
    pub fn duration(&self) -> Duration {
        let one = match self.0 {
            Some(Answer { bench: Some(t), .. }) => t,
            _ => Duration::default()
        };
        let two = match self.1 {
            Some(Answer { bench: Some(t), .. }) => t,
            _ => Duration::default()
        };
        one + two
    }

    #[deprecated]
    /// Returns the optional answer for both parts of this solution.
    ///
    /// Deprecated in favor of comparing answers directly.
    pub fn into_ans(self) -> (Option<String>, Option<String>) {
        (self.0.map(|part| part.ans), self.1.map(|part| part.ans))
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Part 1: {}", match self.0 {
            Some(ref part) => part.to_string(),
            None => "No implemented".to_owned()
        })?;
        writeln!(f, "Part 2: {}", match self.1 {
            Some(ref part) => part.to_string(),
            None => "No implemented".to_owned()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_from_anything_stringy() {
        let answer = Answer::new(6);
        assert_eq!("6", answer.ans());
        let answer = Answer::new("fred");
        assert_eq!("fred", answer.ans());
        let answer = Answer::new("bob".to_owned());
        assert_eq!("bob", answer.ans());

        struct Zed();
        impl ToString for Zed { fn to_string(&self) -> String { "zom".to_owned() } }
        let answer = Answer::new(Zed());
        assert_eq!("zom", answer.ans());
    }

    #[test]
    fn solution_duration_sums_correctly() {
        let solution = Solution::new(
            Some(Answer::with_bench("one", Some(Duration::new(7, 180)))),
            Some(Answer::with_bench("two", Some(Duration::new(9, 37)))),
        );

        assert_eq!(Duration::new(16, 217), solution.duration());
    }
}