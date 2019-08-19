//! Solution for Advent of Code [2018 Day 05](https://adventofcode.com/2018/day/5).

use crate::common::puzzle;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
/// Error type for when a polymer with a non-letter unit type is
/// constructed.
struct NonLetterTypeError;

/// A polymer that may contain consecutive unit type of opposite polarity.
struct Polymer(String);

/// A polymer that is guaranteed no to contain consecutive unit types
/// of opposite polarity.
struct ReducedPolymer(String);

impl Error for NonLetterTypeError {}

impl fmt::Display for NonLetterTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("NonLetterTypeError").finish()
    }
}

impl FromStr for Polymer {
    type Err = NonLetterTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(NonLetterTypeError);
        }

        Ok(Self(s.to_owned()))
    }
}

impl Polymer {
    /// Converts this polymer into a `ReducedPolymer`.
    fn into_reduced(mut self) -> ReducedPolymer {
        let bytes = unsafe { self.0.as_mut_vec() };

        let mut finger = 0;
        while finger + 1 < bytes.len() {
            if letter_opposite_polarity(bytes[finger], bytes[finger + 1]) {
                // Remove opposite-polarity pair from the polymer string
                bytes.drain(finger..=finger + 1);
                // Retract the finger to account for the removal of the
                // byte under the finger, so long as the finger is not
                // currently at the starting position.
                if finger != 0 {
                    finger -= 1;
                }
            } else {
                // Advance the finger
                finger += 1;
            }
        }

        ReducedPolymer(self.0)
    }

    /// Returns a vector os polymers that represent each iteration of this
    /// polymer with on of its unit types removed.
    fn removed_unit_types_vec(&self) -> Vec<Self> {
        let mut unit_types = HashSet::new();
        for type_letter in self.0.bytes() {
            unit_types.insert(type_letter);
        }

        unit_types
            .into_iter()
            .map(|unit| {
                let (lower, upper) = (unit.to_ascii_lowercase(), unit.to_ascii_uppercase());
                let mut s = self.0.clone();
                let bytes_view = unsafe { s.as_mut_vec() };
                bytes_view.retain(|&b| b != lower && b != upper);
                s
            })
            .map(Polymer)
            .collect()
    }
}

impl ReducedPolymer {
    /// Returns the length of this polymer.
    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_string(puzzle)?;

    let polymer = input.trim_end().parse::<Polymer>()?;
    let removed_unit_polymers = polymer.removed_unit_types_vec();

    solve_parts!(
        1 => polymer.into_reduced().len(),
        2 => removed_unit_polymers.into_iter()
                .map(|p| p.into_reduced().len())
                .min()
                .unwrap()
    )
}

/// Returns `true` if the given bytes represent the same ASCII letter, but
/// with different cases.
fn letter_opposite_polarity(first: u8, second: u8) -> bool {
    debug_assert!(first.is_ascii_alphabetic() && second.is_ascii_alphabetic());
    const MASK: u8 = 1 << 5;
    // Check that the letters have the same rank but different cases.
    (first | MASK == second | MASK) && (first & MASK != second & MASK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_opposite_polarity() {
        assert!(letter_opposite_polarity(b'a', b'A'));
        assert!(letter_opposite_polarity(b'A', b'a'));
        assert!(!letter_opposite_polarity(b'a', b'a'));
        assert!(!letter_opposite_polarity(b'A', b'A'));
        assert!(!letter_opposite_polarity(b'a', b'B'));
        assert!(!letter_opposite_polarity(b'B', b'a'));
    }

    #[test]
    fn solution() {
        assert_solution!(11042, 6872, puzzle::Selection::new(2018, 5))
    }

    #[test]
    fn ex1() {
        const TEST_CASES: &[(&str, usize)] = &[("aA", 0), ("abBA", 0), ("abAB", 4), ("aabAAB", 6)];

        for &(polymer, reduced_length) in TEST_CASES {
            assert_eq!(
                polymer.parse::<Polymer>().unwrap().into_reduced().len(),
                reduced_length
            );
        }

        assert_eq!(
            "dabAcCaCBAcCcaDA"
                .parse::<Polymer>()
                .unwrap()
                .into_reduced()
                .0,
            "dabCBAcaDA".to_owned(),
        );
    }

    #[test]
    fn ex2() {
        let polymers = "dabAcCaCBAcCcaDA"
            .parse::<Polymer>()
            .unwrap()
            .removed_unit_types_vec();
        assert_eq!(
            polymers
                .into_iter()
                .map(|p| p.into_reduced().len())
                .min()
                .unwrap(),
            4
        )
    }
}
