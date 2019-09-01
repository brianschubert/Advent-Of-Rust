//! Solution for 2015 Day 04

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};
use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    solve_parts! {
        both => find_first_coin(input.as_bytes())
    }
}

/// Returns first "AdventCoins" generated during part one and part two,
/// respectively,
fn find_first_coin(seed: &[u8]) -> (u64, u64) {
    let mut hasher = Md5::new();
    let mut coin_5 = None;
    let mut coin_6 = None;

    for index in 0..u64::max_value() {
        hasher.input(seed);
        hasher.input(index.to_string().as_bytes());

        let mut result = [0_u8; 16];
        hasher.result(&mut result);

        // Check if first 5 chars in hex representation would be 0
        if u16::from(result[0]) + u16::from(result[1]) + u16::from(result[2] >> 4) == 0 {
            if coin_5.is_none() {
                coin_5 = Some(index);
            }
            // Check if the sixth is also a 0
            if result[2] == 0 && coin_6.is_none() {
                coin_6 = Some(index);
            }
        }

        if let (Some(one), Some(two)) = (coin_5, coin_6) {
            return (one, two);
        }

        hasher.reset();
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(254_575, 1_038_736, Pz::new(2015, 4))
    }

    #[test]
    fn ex1() {
        // Part two causes these to run for ~60s
        assert_eq!(609_043, find_first_coin(b"abcdef").0);
        assert_eq!(1_048_970, find_first_coin(b"pqrstuv").0);
    }
}
