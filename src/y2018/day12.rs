//! Solution for Advent of Code [2018 Day 12](https://adventofcode.com/2018/day/12).

use crate::common::puzzle;
use std::fmt;
use std::fmt::{Debug, Write};
use std::str::FromStr;

const GENERATION_COUNT_1: usize = 20;

const GENERATION_COUNT_2: usize = 5_000_0000_000;

/// The size of a plant generation rule for this puzzle.
const RULE_WIDTH: usize = 5;

/// A sequence of "pots" either either contain (`true`) or do not contain
/// (`false`) a plant.
struct PlantSequence(Vec<bool>);

#[derive(Copy, Clone, Eq, PartialEq)]
/// Ordinal in [0,31] identifying one of the 32 different possible plant patterns
/// for this puzzle.
///
/// # Examples
///
/// - `0` corresponds to the pattern `.....`
/// - `1` corresponds to the pattern `#....`
/// - `2` corresponds to the pattern `.#...`
/// - `3` corresponds to the pattern `##...`
/// - `31` corresponds to the pattern `#####`
struct PlantPattern {
    ord: u8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// A compact representing of the generation rules that describe how
/// the potted plants evolve between generations.
///
/// Each of the 32 bits in this rule set describe the output state of the
/// plant rule with the ordinal value matching the bits index.
///
/// For example, the first bit corresponds the the plant pattern with ordinal
/// value `0`, which represents the pattern "`.....`". A `1` in this position
/// means that the pattern "`.....`" will produce a plant in the next generation,
/// i.e. "`..... => #`". A `0` in this position means that no plant will be
/// produced in the next generation, i.e. "`..... => .`".
///
/// Similarly, third bit corresponds to the plant pattern with ordinal `2`,
/// which represents the pattern "`##...`". A `1` in this position corresponds
/// to the rule "`##... => #`", and a `0` corresponds to the rule
/// "`##... => .`".
struct RuleSet {
    rules: u32,
}

#[derive(Debug)]
struct PlantSimulator {
    /// The location of the central pot in the plant sequence.
    center: usize,
    /// The current generation of potted plants.
    plants: PlantSequence,
    /// The rules that govern how future generation of potted plants are
    /// produced.
    rule_set: RuleSet,
}

impl FromStr for PlantSequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.bytes()
            .map(|b| match b {
                b'.' => Ok(false),
                b'#' => Ok(true),
                _ => Err("invalid character found in initial state"),
            })
            .collect::<Result<_, _>>()
            .map(|seq| Self(seq))
    }
}

impl PlantPattern {
    fn from_slice(plants: &[bool]) -> Result<Self, String> {
        if plants.len() != RULE_WIDTH {
            return Err(format!(
                "plant pattern MUST have a length of {}, {} received",
                RULE_WIDTH,
                plants.len()
            ));
        }
        let mut bits = 0;
        for i in 0..RULE_WIDTH as u8 {
            if plants[i as usize] {
                bits |= 1 << i;
            }
        }
        Ok(Self { ord: bits })
    }
}

impl Debug for PlantPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for i in 0..RULE_WIDTH as u8 {
            f.write_char(if self.ord & (1 << i) == 0 { '.' } else { '#' })?
        }
        Ok(())
    }
}

impl RuleSet {
    fn new() -> Self {
        Self {
            rules: Default::default(),
        }
    }

    fn set_pattern(&mut self, pattern: PlantPattern, result: bool) {
        if result {
            self.rules |= 1 << pattern.ord as u32
        } else {
            self.rules &= !(1 << pattern.ord as u32);
        }
    }

    fn check_pattern(self, pattern: PlantPattern) -> bool {
        self.rules & (1 << pattern.ord as u32) != 0
    }
}

impl Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_char('[')?;
        if f.alternate() {
            f.write_char('\n')?
        }
        for i in 0..32 {
            if f.alternate() {
                f.write_str("\t")?
            }
            f.write_str(&format!(
                "{:?} => {},",
                PlantPattern { ord: i },
                if self.check_pattern(PlantPattern { ord: i }) {
                    '#'
                } else {
                    '.'
                }
            ))?;
            f.write_char(if f.alternate() { '\n' } else { ' ' })?;
        }
        f.write_char(']')
    }
}

impl PlantSimulator {
    fn new(rules: RuleSet, initial_state: PlantSequence) -> Self {
        const PADDING_WIDTH: usize = 4;
        let mut plants: PlantSequence = PlantSequence(Vec::with_capacity(
            initial_state.0.len() + PADDING_WIDTH * 2,
        ));
        // Include four empty pots of padding to handle '....# => #' or '#.... => #' cases
        for _ in 0..PADDING_WIDTH {
            plants.0.push(false);
        }
        plants.0.extend(initial_state.0);
        for _ in 0..PADDING_WIDTH {
            plants.0.push(false);
        }

        Self {
            center: PADDING_WIDTH,
            plants,
            rule_set: rules,
        }
    }

    fn next_generation(&mut self) {
        // The number of pots in the PlantGeneration to skip over when
        // determing what pots contain plants in the next generation.
        const RULE_PADDING: usize = (RULE_WIDTH - 1) / 2;
        let plant_count = self.plants.0.len();

        let mut next_generation: PlantSequence = PlantSequence(vec![false; plant_count]);

        for i in RULE_PADDING..plant_count - RULE_PADDING {
            next_generation.0[i] = self.rule_set.check_pattern(
                PlantPattern::from_slice(&self.plants.0[i - RULE_PADDING..=i + RULE_PADDING])
                    .unwrap(),
            );
        }

        // If generation produced a plant in the leftmost checked slot,
        // add an additional empty pot to account for the expanding
        // size of the pot sequence.
        if next_generation.0[RULE_PADDING] {
            next_generation.0.insert(0, false);
            self.center += 1;
        }

        // If generation produced a plant in the rightmost checked slot,
        // add an additional empty pot to account for the expanding
        // size of the pot sequence.
        if next_generation.0[plant_count - RULE_PADDING - 1] {
            next_generation.0.push(false);
        }
        self.plants = next_generation
    }

    /// Computes the signed sum of the indices of the pots in this simulator
    /// that currently contain a plant.
    fn pot_checksum(&self) -> isize {
        self.plants
            .0
            .iter()
            .enumerate()
            .filter(|&(_, &has_plant)| has_plant)
            .map(|(i, _)| i as isize - self.center as isize)
            .sum()
    }
}

impl Debug for PlantSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(
            &self
                .0
                .iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>(),
        )
    }
}

fn parse_input<S: AsRef<str>>(input: S) -> Result<PlantSimulator, &'static str> {
    const INITIAL_STATE_START: usize = "initial state: ".len();
    let mut input_lines = input.as_ref().lines();

    // Parse first line of the input as the initial state
    let initial_state: PlantSequence =
        input_lines.next().ok_or("input must not be empty")?[INITIAL_STATE_START..].parse()?;
    let mut rule_set = RuleSet::new();

    // Parse all lines after line 2 as the plant rules
    for l in input_lines.skip(1) {
        let pattern = l[0..RULE_WIDTH]
            .parse::<PlantSequence>()
            .map(|pat| PlantPattern::from_slice(&pat.0[..]).unwrap())?;
        rule_set.set_pattern(pattern, l.as_bytes()[9] == b'#');
    }
    Ok(PlantSimulator::new(rule_set, initial_state))
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_string(puzzle)?;

    let solver = |generations: usize| -> Result<isize, &'static str> {
        let mut sim = parse_input(&input)?;
        for _ in 0..generations {
            sim.next_generation();
        }
        Ok(sim.pot_checksum())
    };

    solve_parts!(
        1 => { solver(GENERATION_COUNT_1)? },
        // Part 2 still to be solved
        // The current approach is too inefficient to compute the solution
        // in a reasonable amount of time.
        2 => { solver(GENERATION_COUNT_2)? },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    #[ignore]
    fn solution() {
        assert_solution!(3915, puzzle::Selection::new(2018, 12));
    }

    #[test]
    fn ex1() {
        let mut sim = parse_input(DEMO_INPUT).unwrap();
        for _ in 0..GENERATION_COUNT_1 {
            sim.next_generation();
        }
        assert_eq!(325, sim.pot_checksum());
    }
}
