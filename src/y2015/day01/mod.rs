//! Solution for 2015 Day 01

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    let moves = parse_elevator_offset(input.as_bytes());

    solve_parts! {
        1 => final_floor(&moves),
        2 => first_basement_pos(&moves)?
    }
}

/// Converts a byte slice into elevator offsets
fn parse_elevator_offset(tokens: &[u8]) -> Vec<i8> {
    tokens.iter().map(|&b| match b {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }).collect()
}

/// Returns the final floor one arrives on after following the specified
/// elevator tokens.
fn final_floor(moves: &[i8]) -> i16 {
    moves.iter().map(|&b| i16::from(b)).sum()
}

/// Returns the position of the first move token that causes one to
/// enter the basement i.e. enter a negative index.
fn first_basement_pos(tokens: &[i8]) -> Result<usize, &'static str> {
    let mut floor = 0_i16;
    for (pos, &offset) in tokens.iter().enumerate() {
        floor += i16::from(offset);
        if floor == -1 {
            return Ok(pos + 1);
        }
    }
    Err("tokens never lead to basement")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(74, 1795, Pz::new(2015, 1))
    }

    #[test]
    fn ex1() {
        let test_cases: [(i8, &'static [u8]); 9] = [
            (0, b"(())"),
            (0, b"()()"),
            (3, b"((("),
            (3, b"(()(()("),
            (3, b"))((((("),
            (-1, b"())"),
            (-1, b"))("),
            (-3, b")())())"),
            (-3, b")))"),
        ];

        for &(expected, input) in test_cases.iter() {
            let moves = parse_elevator_offset(input);
            assert_eq!(i16::from(expected), final_floor(&moves));
        }
    }

    #[test]
    fn ex2() {
        let test_cases: [(usize, &'static [u8]); 2] = [(1, b")"), (5, b"()())")];

        for &(expected, input) in test_cases.iter() {
            let moves = parse_elevator_offset(input);
            assert_eq!(expected, first_basement_pos(&moves).unwrap());
        }
    }
}
