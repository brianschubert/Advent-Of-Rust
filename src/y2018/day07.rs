//! Solution for Advent of Code [2018 Day 07](https://adventofcode.com/2018/day/7).

use crate::common::puzzle;
use std::collections::HashSet;
use std::str::FromStr;

/// The maximum number of steps that can exist.
const MAX_STEP_COUNT: usize = 26;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An step identifier.
struct StepID(u8);

#[derive(Debug, Copy, Clone)]
/// A description of a single dependencies between two steps.
struct DependencyEntry {
    target: StepID,
    requires: StepID,
}

/// An iterator over a sequence of dependent steps.
///
/// Solves Part 1 of the problem.
struct StepSorter {
    /// A letter-ordinal indexed mapping between `StepID`s and their
    /// unsatisfied dependencies.
    ///
    /// `Some(HashSet)` at position `i` indicates that the step with
    /// ID `(i + b'A') as char` is waiting for the steps in the hash
    /// set to be completed.
    ///
    /// `None` at position `i` indicates that the step with ID
    /// `(i + b'A') as char` has already been completed and need
    /// not be considered when determining the next step to complete.
    requirements_map: [Option<HashSet<StepID>>; MAX_STEP_COUNT],
}

impl StepID {
    /// Converts the specified ASCII byte into a step ID.
    ///
    /// Panics if the given byte is no an ASCII uppercase letter.
    fn from_ascii_unsafe(b: u8) -> Self {
        if !b.is_ascii_uppercase() {
            panic!("StepID must be a valid ASCII uppercase letter");
        }
        Self(b - b'A')
    }

    /// Converts the specified letter index into a step ID.
    ///
    /// Panics if the given index is not in `0..26`.
    fn from_letter_index(i: usize) -> Self {
        if i > 25 {
            panic!("StepID must be a valid ASCII uppercase letter");
        };
        Self(i as u8)
    }

    #[inline]
    /// Returns this step ID as a letter index.
    fn as_index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    /// Returns this step ID as an ASCII uppercase letter.
    fn as_char(self) -> char {
        (self.0 + b'A') as char
    }
}

impl FromStr for DependencyEntry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const REQUIREMENT_POS: usize = 5;
        const TARGET_POS: usize = 36;
        let bytes = s.as_bytes();

        let requires = StepID::from_ascii_unsafe(bytes[REQUIREMENT_POS]);
        let target = StepID::from_ascii_unsafe(bytes[TARGET_POS]);

        Ok(Self { target, requires })
    }
}

impl StepSorter {
    fn from_dependencies(steps: &[DependencyEntry]) -> Self {
        let mut requirements_map: [Option<HashSet<StepID>>; MAX_STEP_COUNT] = Default::default();
        for &step in steps {
            let req_set = requirements_map[step.target.as_index()].get_or_insert_with(HashSet::new);
            req_set.insert(step.requires);
            // Ensure the step with no dependencies is `Some` with an empty `HashSet` rather than `None`
            requirements_map[step.requires.as_index()].get_or_insert_with(HashSet::new);
        }
        Self { requirements_map }
    }
}

impl Iterator for StepSorter {
    type Item = StepID;

    fn next(&mut self) -> Option<Self::Item> {
        let next_step = self
            .requirements_map
            .iter()
            .enumerate()
            // Map (usize, &Option<HashSet>) to (usize, &HashSet)
            .filter_map(|(i, maybe_req)| maybe_req.as_ref().map(|req_set| (i, req_set)))
            // Find all the sets that have no unsatisfied dependencies
            .filter(|&(_, req_set)| req_set.is_empty())
            .map(|(letter_index, _)| letter_index)
            // Find the ready step that comes first alphabetically
            .min();

        // If a next step was found, remove it the array of incomplete steps,
        // and then remove it from the unsatisfied dependencies of all other
        // incomplete steps.
        next_step.map(|letter_index| {
            let step_id = StepID::from_letter_index(letter_index);

            self.requirements_map[letter_index] = None;

            for step_reqs in self.requirements_map.iter_mut() {
                if let Some(req_set) = step_reqs {
                    req_set.remove(&step_id);
                }
            }
            step_id
        })
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_lines(puzzle)?;
    let dependencies: Vec<DependencyEntry> = input.iter().map(|s| s.parse().unwrap()).collect();

    solve_parts!(
        1 => StepSorter::from_dependencies(&dependencies)
                .map(StepID::as_char)
                .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {}

    #[test]
    fn ex1() {
        const STEP_DESCRIPTION: &[&str] = &[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];
        let dependencies: Vec<DependencyEntry> = STEP_DESCRIPTION
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let step_order: String = StepSorter::from_dependencies(&dependencies)
            .map(StepID::as_char)
            .collect();
        assert_eq!(step_order, "CABDFE")
    }

    #[test]
    fn ex2() {}
}
