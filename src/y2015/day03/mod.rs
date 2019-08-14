//! Solution for 2015 Day 03

use std::collections::HashSet;
use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};
use common::util::Pt;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = parse_input(&pio::fetch_string(puzzle)?)?;

    solve_parts! {
        1 => visit_houses(&input),
        2 => visit_houses_parallel(&input)
    }
}

/// Returns the total number of houses visited when taking the
/// specified route (Part One).
fn visit_houses(moves: &[Pt<i16>]) -> usize {
    let mut pos = Pt::origin();
    let mut visited: HashSet<Pt<i16>> = HashSet::new();
    visited.insert(pos);

    for instr in moves {
        pos += *instr;
        visited.insert(pos);
    }

    visited.len()
}

/// Returns the total number of houses visited when two entities
/// alternate taking instructions from the specified route (Part Two).
fn visit_houses_parallel(moves: &[Pt<i16>]) -> usize {
    let mut pos_santa = Pt::origin();
    let mut pos_robot = Pt::origin();
    let mut visited: HashSet<Pt<i16>> = HashSet::new();
    visited.insert(pos_santa);

    for instr in moves.chunks(2) {
        pos_santa += instr[0];
        pos_robot += instr[1]; // panics if input length is odd


        visited.insert(pos_santa);
        visited.insert(pos_robot);
    }

    visited.len()
}

/// Parses the specified string into direction offsets.
fn parse_input<S: AsRef<str>>(input: S) -> Result<Vec<Pt<i16>>, &'static str> {
    input.as_ref().bytes().map(|b| match b {
        b'^' => Ok(Pt::n()),
        b'>' => Ok(Pt::e()),
        b'v' => Ok(Pt::s()),
        b'<' => Ok(Pt::w()),
        _ => Err("invalid direction token")
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            2572,
            2631,
            Pz::new(2015, 3)
        )
    }

    #[test]
    fn ex_both() {
        let test_cases: [(usize, usize, &'static str); 3] = [
            (3, 3, "^>"),
            (4, 3, "^>v<"),
            (2, 11, "^v^v^v^v^v")
        ];

        for &(alone, with_robot, input) in test_cases.iter() {
            let moves = parse_input(input).unwrap();
            assert_eq!(alone, visit_houses(&moves));
            assert_eq!(with_robot, visit_houses_parallel(&moves));
        }
    }
}