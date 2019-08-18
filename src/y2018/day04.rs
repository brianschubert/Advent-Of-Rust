//! Solution for Advent of Code [2018 Day 04](https://adventofcode.com/2018/day/4).

use crate::common::puzzle::{input as pio, PuzzleResult, PuzzleSelection as Pz, Solution};

#[cfg(test)]
const EXAMPLE_INPUT: &[&str] = &[
    "[1518-11-01 00:00] Guard #10 begins shift",
    "[1518-11-01 00:05] falls asleep",
    "[1518-11-01 00:25] wakes up",
    "[1518-11-01 00:30] falls asleep",
    "[1518-11-01 00:55] wakes up",
    "[1518-11-01 23:58] Guard #99 begins shift",
    "[1518-11-02 00:40] falls asleep",
    "[1518-11-02 00:50] wakes up",
    "[1518-11-03 00:05] Guard #10 begins shift",
    "[1518-11-03 00:24] falls asleep",
    "[1518-11-03 00:29] wakes up",
    "[1518-11-04 00:02] Guard #99 begins shift",
    "[1518-11-04 00:36] falls asleep",
    "[1518-11-04 00:46] wakes up",
    "[1518-11-05 00:03] Guard #99 begins shift",
    "[1518-11-05 00:45] falls asleep",
    "[1518-11-05 00:55] wakes up",
];

mod guard;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let lines = pio::fetch_lines(puzzle)?;
    let log = guard::GuardLog::parse_lines(&lines)?;
    solve_parts!(
        1 => {
            let (most_sleepy_guard, most_slept_minute) = log.compute_most_sleepy_guard();
            u32::from(most_sleepy_guard) * most_slept_minute as u32
        },
        2 => {
            let (guard, minute) = log.compute_guard_most_frequently_asleep_same_minute();
            u32::from(guard) * minute as u32
        }

    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(95199, 7887, Pz::new(2018, 4))
    }

    #[test]
    fn ex_both() {
        let log = guard::GuardLog::parse_lines(EXAMPLE_INPUT).unwrap();

        let (most_sleepy_guard, most_slept_minute) = log.compute_most_sleepy_guard();

        assert_eq!((most_sleepy_guard, most_slept_minute), (10, 24));

        assert_eq!(
            log.compute_guard_most_frequently_asleep_same_minute(),
            (99, 45)
        );
    }
}
