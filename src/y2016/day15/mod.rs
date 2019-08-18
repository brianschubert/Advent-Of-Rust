//! Solution for 2016 Day 15
use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

use std::str::FromStr;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let mut input: Vec<Disc> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    solve_parts! {
        1 => required_delay(&input),
        2 => {
            input.push(Disc { pos: 0, range: 11 });
            required_delay(&input)
        }
    }
}

/// A Disc from the puzzle's input
struct Disc {
    /// This Disc's start position at time=0
    pos: u8,
    /// The number of possible position for this Disc.
    range: u8,
}

impl FromStr for Disc {
    type Err = &'static str;

    // Assumes disc depth falls in [0,9] and range falls in [0, 99]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Disc {
            pos: s[54..s.len() - 1]
                .trim_start()
                .parse()
                .map_err(|_| "malformed disc starting position")?,
            range: s[12..14]
                .trim_end()
                .parse()
                .map_err(|_| "malformed disc depth")?,
        })
    }
}

/// Returns the "delay" one must wait in order for a ball to
/// successfully fall through all the specified discs.
fn required_delay(discs: &[Disc]) -> u32 {
    let mut delay = 0_u32;
    loop {
        let check = |(depth, disc): (usize, &Disc)| {
            (1 + delay + depth as u32 + u32::from(disc.pos)) % u32::from(disc.range) != 0
        };
        if !discs.iter().enumerate().any(check) {
            break delay;
        }
        delay += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(122_318, 3_208_583, Pz::new(2016, 15))
    }

    #[test]
    fn ex1() {
        let discs = [
            "Disc #1 has 5 positions; at time=0, it is at position 4.",
            "Disc #2 has 2 positions; at time=0, it is at position 1.",
        ];

        let delay = required_delay(
            &discs.iter().map(|&l| l.parse().unwrap()).collect::<Vec<_>>()
        );

        assert_eq!(5, delay);
    }

    #[test]
    fn parse_disc() {
        let disc: Disc = "Disc #1 has 5 positions; at time=0, it is at position 4.".parse().unwrap();
        assert_eq!(5, disc.range);
        assert_eq!(4, disc.pos);

        let disc: Disc = "Disc #1 has 50 positions; at time=0, it is at position 48.".parse().unwrap();
        assert_eq!(50, disc.range);
        assert_eq!(48, disc.pos);
    }
}
