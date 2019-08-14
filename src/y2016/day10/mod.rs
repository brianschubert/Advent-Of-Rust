//! Solution for 2016 Day 10

use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

use regex::Regex;

use std::collections::{HashMap, BTreeMap};

/// Regex pattern for a bot's pass instruction
const PATTERN_PASS: &str =
    "^bot (\\d{1,3}) gives low to (bot|output) (\\d{1,3}) and high to (bot|output) (\\d{1,3})$";

/// Regex pattern for a bot initializer instruction
const PATTERN_INIT: &str = "^value (\\d{1,3}) goes to bot (\\d{1,3})$";

/// Chip combination to watch for in part one.
const WATCHED_CHIPS: (u8, u8) = (17, 61);

mod bot {
    /// A microchip value
    pub type ChipValue = u8;

    /// Destination for a bot to pass its chips to.
    pub enum ChipDest {
        Bot(u8),
        Output(u8),
    }

    /// A bot carrying at most two ordered chips.
    pub struct Bot {
        low: Option<ChipValue>,
        high: Option<ChipValue>,
    }

    impl Bot {
        /// Builds a new bot with no chips.
        pub fn new() -> Self {
            Bot { low: None, high: None }
        }

        /// Passes a chip to a bot.
        ///
        /// Panics if the bots "hands" are full i.e. if it already has
        /// two chips.
        pub fn give_chip(&mut self, chip: ChipValue) {
            if self.high.is_some() {
                panic!("bot already has its hands full!");
            }

            match self.low {
                Some(current) => if current < chip {
                    self.high = Some(chip)
                } else {
                    self.high = self.low.take();
                    self.low = Some(chip);
                }
                None => self.low = Some(chip)
            }
        }

        /// Returns true is the bots is carrying two chips.
        pub fn can_pass(&self) -> bool {
            self.high.is_some()
        }

        /// Removes a bots chips.
        ///
        /// Panics is either chip is missing.
        pub fn take_chips(&mut self) -> (ChipValue, ChipValue) {
            (
                self.low.take().expect("missing low chip"),
                self.high.take().expect("missing high chip")
            )
        }
    }
}

use self::bot::*;

/// Description of where a bot should send its chips once its "hands"
/// are full.
pub struct PassDirective {
    bot: u8,
    low: ChipDest,
    high: ChipDest,
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let (pass_instr, mut bots) = parse_input(
        &mut pio::fetch_lines(puzzle)?
    );

    solve_parts! {
        both => pass_chips(&pass_instr, &mut bots, WATCHED_CHIPS)
    }
}

fn parse_input<T>(lines: &[T]) -> (Vec<PassDirective>, HashMap<u8, Bot>)
    where T: AsRef<str>
{
    let re_pass = Regex::new(PATTERN_PASS).unwrap();
    let re_init = Regex::new(PATTERN_INIT).unwrap();

    let mut pass_instr = Vec::new();
    let mut bots = HashMap::new();

    for line in lines {
        let line = line.as_ref();

        if let Some(pass) = re_pass.captures(line) {
            let bot = pass.get(1).unwrap().as_str().parse().unwrap();
            let low = {
                let id = pass.get(3).unwrap().as_str().parse().unwrap();
                match pass.get(2).unwrap().as_str() {
                    "output" => ChipDest::Output(id),
                    "bot" => ChipDest::Bot(id),
                    _ => panic!("unknown low chip destination")
                }
            };
            let high = {
                let id = pass.get(5).unwrap().as_str().parse().unwrap();
                match pass.get(4).unwrap().as_str() {
                    "output" => ChipDest::Output(id),
                    "bot" => ChipDest::Bot(id),
                    _ => panic!("unknown high chip destination")
                }
            };
            pass_instr.push(PassDirective { bot, low, high });
        } else if let Some(init) = re_init.captures(line) {
            bots.entry(init.get(2).unwrap().as_str().parse().unwrap())
                .or_insert_with(Bot::new)
                .give_chip(init.get(1).unwrap().as_str().parse().unwrap());
        } else {
            panic!(format!("Malformed line: {}", line))
        }
    }

    (pass_instr, bots)
}

/// Transfers chips between bots (and outputs) according to the specified
/// pass directives.
///
/// Returns a tuple containing (1) the bot responsible for comparing the watched
/// chips and (2) the product of the first chip in outputs `0`, `1`, and `2`.
fn pass_chips(
    pass_instr: &[PassDirective],
    bots: &mut HashMap<u8, Bot>,
    watch_for: (ChipValue, ChipValue),
) -> (u8, u32) {
    let mut watched_bot: Option<u8> = None;
    let mut outputs = BTreeMap::new();

    loop {
        // Copy the ids of the bots with two chips
        let ready_bots: Vec<u8> = bots
            .iter()
            .filter(|e| e.1.can_pass())
            .map(|e| *e.0)
            .collect();

        if ready_bots.is_empty() {
            break;
        }

        for bot_id in ready_bots {
            let chips = bots.get_mut(&bot_id).unwrap().take_chips();

            if watched_bot.is_none() && watch_for == chips {
                watched_bot = Some(bot_id);
            }

            let directive = pass_instr.iter()
                .find(|p| p.bot == bot_id)
                .expect("no pass instruction found");

            match directive.low {
                ChipDest::Bot(id) => bots.entry(id)
                    .or_insert_with(Bot::new)
                    .give_chip(chips.0),
                ChipDest::Output(id) => outputs.entry(id)
                    .or_insert_with(Vec::new)
                    .push(chips.0)
            }

            match directive.high {
                ChipDest::Bot(id) => bots.entry(id)
                    .or_insert_with(Bot::new)
                    .give_chip(chips.1),
                ChipDest::Output(id) => outputs.entry(id)
                    .or_insert_with(Vec::new)
                    .push(chips.1)
            }
        }
    }

    (
        watched_bot.expect(
            &format!("No bot found comparing chips {:?}", &watch_for)[..]
        ),
        outputs
            .range(0..3)
            .map(|(_, out)| u32::from(*out.first().unwrap()))
            .product()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution! {
            141,
            1209,
            Pz::new(2016, 10)
        }
    }

    #[test]
    fn ex_both() {
        let instr: [&'static str; 6] = [
            "value 5 goes to bot 2",
            "bot 2 gives low to bot 1 and high to bot 0",
            "value 3 goes to bot 1",
            "bot 1 gives low to output 1 and high to bot 0",
            "bot 0 gives low to output 2 and high to output 0",
            "value 2 goes to bot 2"
        ];

        let (watched_bot, output_product) = {
            let (instr, mut bots) = parse_input(&instr);
            pass_chips(&instr, &mut bots, (2, 5))
        };

        assert_eq!(2, watched_bot);
        assert_eq!(30, output_product);
    }
}
