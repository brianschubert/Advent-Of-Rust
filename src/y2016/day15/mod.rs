//! # Solution for 2016 Day 15
use common::{input as pio, PuzzleSelection as Pz, Solution};

use std::str::FromStr;

pub fn solve(puzzle: Pz) -> Solution {
    let mut input: Vec<Disc> = pio::fetch_lines(puzzle)
        .expect("input file could not be read")
        .into_iter()
        .map(|line| line.parse().expect("malformed input line"))
        .collect();

    solve_parts! {
        1 => required_delay(&input),
        2 => {
            input.push(Disc { pos: 0, range: 11 });
            required_delay(&input)
        }
    }
}

/// A Disc from the puzzle's input
struct Disc{
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
                .trim_left()
                .parse()
                .map_err(|_|"malformed disc starting position")?,
            range: s[12..14]
                .trim_right()
                .parse()
                .map_err(|_|"malformed disc depth")?,
        })
    }
}

/// Returns the "delay" one must wait in order for a ball to
/// successfully fall through all the specified discs.
fn required_delay(discs: &[Disc]) -> u32 {
    let mut delay = 0_u32;
    loop {
        if !discs
            .iter()
            .enumerate()
            .any(|(depth, ref disc)|{
                (1 + delay + depth as u32 + disc.pos as u32) % disc.range as u32 != 0
            }) {

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
        assert_solution!(
            122318,
            3208583,
            Pz::of(2016, 15)
        )
    }

    #[test]
    fn ex1() {
        let discs = [
            "Disc #1 has 5 positions; at time=0, it is at position 4.",
            "Disc #2 has 2 positions; at time=0, it is at position 1."
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
