//! Solution for Advent of Code [2018 Day 01](https://adventofcode.com/2018/day/1).

use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};
use std::collections::HashSet;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input: Vec<i32> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;


    solve_parts!(
        1 => input.iter().sum::<i32>(),
        2 => find_first_repeated_frequency(&input)
    )
}

fn find_first_repeated_frequency(freq_changes: &[i32]) -> i32 {
    let mut history = HashSet::new();
    let mut current_freq = 0;

    loop {
        for &delta in freq_changes {
            let already_seen = !history.insert(current_freq);
            if already_seen {
                return current_freq;
            }
            current_freq += delta;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            518,
            72889,
            Pz::new(2018, 1)
        )
    }

    #[test]
    fn ex2() {
        let test_cases: [(&[i32], i32); 4] = [
            (&[1, -1], 0),
            (&[3, 3, 4, -2, -4], 10),
            (&[-6, 3, 8, 5, -6], 5),
            (&[7, 7, -2, -7, -4], 14),
        ];

        for (input, expected) in test_cases.into_iter() {
            assert_eq!(
                find_first_repeated_frequency(&input),
                *expected,
            )
        }
    }
}
