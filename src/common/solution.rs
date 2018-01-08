use std::string::ToString;
use std::time::Duration;

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
    /// Builds an answer with the specified output and optional benchmark.
    pub fn new<S>(ans: S, bench: Option<Duration>) -> Self
        where S: ToString
    {
        Answer { ans: ans.to_string(), bench }
    }

    /// Returns a slice to this puzzle part's answer string.
    pub fn ans(&self) -> &str {
        &self.ans[..]
    }

    /// Returns a reference to this puzzle part's optional benchmark.
    pub fn bench(&self) -> &Option<Duration> {
        &self.bench
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Answer) -> bool {
        self.ans == other.ans
    }
}

impl<T> From<T> for Answer
    where T: ToString
{
    fn from(other: T) -> Self {
        Answer { ans: other.to_string(), bench: None }
    }
}

impl Solution {
    /// Builds a solution from the specified optional answers.
    pub fn new<T, U>(one: T, two: U) -> Solution
        where T: Into<Option<Answer>>,
              U: Into<Option<Answer>>
    {
        Solution(one.into(), two.into())
    }

    /// Builds a solution with `None` for both answers.
    pub fn empty() -> Self {
        Solution(None, None)
    }

    #[deprecated]
    /// Returns the optional answer for both parts of this solution.
    ///
    /// Deprecated in favor of comparing answers directly.
    pub fn extract_ans(self) -> (Option<String>, Option<String>) {
        (self.0.and_then(|a| Some(a.ans)), self.1.and_then(|a| Some(a.ans)))
    }
}
