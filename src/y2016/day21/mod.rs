//! Solution for 2016 Day 21

use common::puzzle::{input as pio, PuzzleResult, PuzzleSelection as Pz, Solution};

/// Bytes to be scrambled according to the input during part one.
const BYTES_TO_SCRAMBLE: &[u8; 8] = b"abcdefgh";

/// Bytes to be unscrambled according to the input during part two.
const BYTES_TO_UNSCRAMBLE: &[u8; 8] = b"fbgdceah";

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input: Vec<scrambler::ScrambleRule> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    solve_parts! {
        1 => {
            let mut word = scrambler::WordScrambler::new(BYTES_TO_SCRAMBLE);
            for rule in input.iter() {
                word.apply_rule(rule).expect("failed to apply rule");
            }
            word.into_word()
        },
        2 => {
            let mut word = scrambler::WordScrambler::new(BYTES_TO_UNSCRAMBLE);
            for rule in input.iter().rev() {
                word.reverse_rule(rule).expect("failed to apply rule");
            }
            word.into_word()
        }
    }
}

mod scrambler {
    use common::util::RotateSigned;
    use std::{fmt, str};
    use std::error::Error;

    #[derive(Debug)]
    /// An error that occurs while parsing a scramble rule.
    pub struct ScrambleRuleParseError {
        rule: String,
        reason: &'static str,
    }

    impl Error for ScrambleRuleParseError {
        fn description(&self) -> &str {
            &self.reason
        }
    }

    impl fmt::Display for ScrambleRuleParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Failed to parse rule: {}", self.reason)
        }
    }

    /// A rule describing how to scramble the bytes within a word.
    pub enum ScrambleRule {
        SwapPos(usize, usize),
        SwapLet(u8, u8),
        RotByPos { mag: isize },
        RotByLet { det: u8 },
        RevRange { start: usize, end: usize },
        Move { target: usize, dest: usize },
    }

    impl ScrambleRule {
        pub fn is_own_reverse(&self) -> bool {
            match *self {
                ScrambleRule::SwapPos(..) | ScrambleRule::SwapLet(..)
                | ScrambleRule::RevRange { .. } => true,
                _ => false
            }
        }
    }

    impl str::FromStr for ScrambleRule {
        type Err = ScrambleRuleParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s.as_bytes();
            // Match rules by unique columns; assumes correct formatting
            Ok(match bytes[7] {
                b's' => match bytes[0] {
                    b's' => ScrambleRule::SwapPos(
                        s[14..15].parse()
                            .map_err(|_| ScrambleRuleParseError {
                                rule: s.to_owned(),
                                reason: "malformed swap index one",
                            })?,
                        s[30..31].parse()
                            .map_err(|_| ScrambleRuleParseError {
                                rule: s.to_owned(),
                                reason: "malformed swap index two",
                            })?,
                    ),
                    b'm' => ScrambleRule::Move {
                        target: s[14..15].parse()
                            .map_err(|_| ScrambleRuleParseError {
                                rule: s.to_owned(),
                                reason: "malformed move target",
                            })?,
                        dest: s[28..29].parse()
                            .map_err(|_| ScrambleRuleParseError {
                                rule: s.to_owned(),
                                reason: "malformed move dest",
                            })?,
                    },
                    _ => return Err(ScrambleRuleParseError {
                        rule: s.to_owned(),
                        reason: "unknown scramble rule",
                    })
                },
                b't' => ScrambleRule::SwapLet(bytes[12], bytes[26]),
                b'l' => ScrambleRule::RotByPos {
                    mag: -s[12..13].parse()
                        .map_err(|_| ScrambleRuleParseError {
                            rule: s.to_owned(),
                            reason: "malformed left rot mag",
                        })?,
                },
                b'r' => ScrambleRule::RotByPos {
                    mag: s[13..14].parse()
                        .map_err(|_| ScrambleRuleParseError {
                            rule: s.to_owned(),
                            reason: "malformed right rot mag",
                        })?,
                },
                b'b' => ScrambleRule::RotByLet { det: bytes[35] },
                b' ' => ScrambleRule::RevRange {
                    start: s[18..19].parse()
                        .map_err(|_| ScrambleRuleParseError {
                            rule: s.to_owned(),
                            reason: "malformed rev range start",
                        })?,
                    end: s[28..29].parse()
                        .map_err(|_| ScrambleRuleParseError {
                            rule: s.to_owned(),
                            reason: "malformed rev range end",
                        })?,
                },
                _ => return Err(ScrambleRuleParseError {
                    rule: s.to_owned(),
                    reason: "unknown scramble rule",
                })
            })
        }
    }

    /// Scrambles the bytes in a word.
    pub struct WordScrambler {
        word_bytes: Vec<u8>,
    }

    impl WordScrambler {
        /// Builds a new word scrambler for the specified word.
        pub fn new(word: &[u8]) -> Self {
            WordScrambler {
                word_bytes: Vec::from(word)
            }
        }

        /// Scrambles this scrambler's word according to the specified
        /// rule.
        pub fn apply_rule(
            &mut self,
            rule: &ScrambleRule,
        ) -> Result<(), &'static str> {
            match *rule {
                ScrambleRule::SwapPos(one, two) => self.swap_pos(one, two),
                ScrambleRule::SwapLet(let_one, let_two) => {
                    let one = self.index_of(let_one)
                        .ok_or("no such letter in word")?;
                    let two = self.index_of(let_two)
                        .ok_or("no such letter in word")?;
                    self.swap_pos(one, two);
                }
                ScrambleRule::RotByPos { mag } => self.rotate(mag),
                ScrambleRule::RotByLet { det } => {
                    let mut mag = self.index_of(det)
                        .ok_or("no such letter in word")? as isize;
                    mag += if mag >= 4 { 2 } else { 1 };
                    self.rotate(mag);
                }
                ScrambleRule::RevRange { start, end } =>
                    self.word_bytes[start..=end].reverse(),
                ScrambleRule::Move { target, dest } => {
                    let payload = self.word_bytes.remove(target);
                    self.word_bytes.insert(dest, payload);
                }
            }
            Ok(())
        }

        /// Reverses prior scrambling to this scrambler's word
        /// according to the specified rule.
        pub fn reverse_rule(
            &mut self,
            rule: &ScrambleRule,
        ) -> Result<(), &'static str> {
            if rule.is_own_reverse() {
                self.apply_rule(rule)?;
            } else {
                match *rule {
                    ScrambleRule::RotByPos { mag } => self.rotate(-mag),
                    ScrambleRule::RotByLet { det } => {
                        let pos = self.index_of(det)
                            .ok_or("no such letter in word")?;
                        let mag = self.find_prior_let_rot(pos);
                        self.rotate(mag)
                    }
                    ScrambleRule::Move { target, dest } => {
                        let payload = self.word_bytes.remove(dest);
                        self.word_bytes.insert(target, payload);
                    }
                    _ => unreachable!()
                }
            }
            Ok(())
        }

        #[cfg(test)]
        /// Returns a reference to this scrambler's word's bytes.
        pub fn word_bytes(&self) -> &[u8] {
            &self.word_bytes
        }

        /// Converts this scrambler into a string of its underlying
        /// word.
        pub fn into_word(self) -> String {
            String::from_utf8(self.word_bytes)
                .expect("invalid utf8 in word bytes ")
        }

        /// Returns the index of the specified `byte` in this
        /// scrambler's word.
        fn index_of(&self, byte: u8) -> Option<usize> {
            self.word_bytes.iter().position(|&b| b == byte)
        }

        /// Swaps the bytes at the specified indexes in this
        /// scrambler's word.
        fn swap_pos(&mut self, one: usize, two: usize) {
            let buf = self.word_bytes[one];
            self.word_bytes[one] = self.word_bytes[two];
            self.word_bytes[two] = buf;
        }

        /// Rotates the bytes in this scramblers word byte the
        /// specified magnitude.
        fn rotate(&mut self, mag: isize) {
            self.word_bytes.rotate_signed(mag)
        }

        /// Determines the rotation required to restore a rotation
        /// by letter scramble.
        ///
        /// *NOTE*: Short words cannot be recovered as multiple
        /// indexes may produce the same rotation. For example,
        /// with a 5-byte word, a rotation about a letter at either
        /// index `1` or index `4` will result in the byte landing
        /// at index `2`.
        fn find_prior_let_rot(&self, cur_pos: usize) -> isize {
            let len = self.word_bytes.len();
            for i in 0..len {
                if (2 * i + if i >= 4 { 2 } else { 1 }) % len == cur_pos {
                    return i as isize - cur_pos as isize;
                }
            }
            panic!("impossible letter pos")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_scramble_rules() {
            match "swap position 7 with position 6".parse().unwrap() {
                ScrambleRule::SwapPos(one, two) => {
                    assert_eq!(7, one);
                    assert_eq!(6, two);
                }
                _ => panic!("failed to parse position swap")
            }

            match "swap letter g with letter f".parse().unwrap() {
                ScrambleRule::SwapLet(one, two) => {
                    assert_eq!(b'g', one);
                    assert_eq!(b'f', two);
                }
                _ => panic!("failed to parse letter swap")
            }

            match "rotate left 2 steps".parse().unwrap() {
                ScrambleRule::RotByPos { mag } => {
                    assert_eq!(-2, mag);
                }
                _ => panic!("failed to parse left rot")
            }

            match "rotate right 6 steps".parse().unwrap() {
                ScrambleRule::RotByPos { mag } => {
                    assert_eq!(6, mag);
                }
                _ => panic!("failed to parse left rot")
            }

            match "rotate based on position of letter a".parse().unwrap() {
                ScrambleRule::RotByLet { det } => {
                    assert_eq!(b'a', det);
                }
                _ => panic!("failed to parse left rot")
            }

            match "reverse positions 3 through 4".parse().unwrap() {
                ScrambleRule::RevRange { start, end } => {
                    assert_eq!(3, start);
                    assert_eq!(4, end);
                }
                _ => panic!("failed to parse left rot")
            }

            match "move position 1 to position 5".parse().unwrap() {
                ScrambleRule::Move { target, dest } => {
                    assert_eq!(1, target);
                    assert_eq!(5, dest);
                }
                _ => panic!("failed to parse left rot")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            "fdhbcgea",
            "egfbcadh",
            Pz::new(2016, 21)
        );
    }

    #[test]
    fn ex1() {
        let rules: [(&'static str, &'static [u8; 5]); 8] = [
            ("swap position 4 with position 0", b"ebcda"),
            ("swap letter d with letter b", b"edcba"),
            ("reverse positions 0 through 4", b"abcde"),
            ("rotate left 1 step", b"bcdea"),
            ("move position 1 to position 4", b"bdeac"),
            ("move position 3 to position 0", b"abdec"),
            ("rotate based on position of letter b", b"ecabd"),
            ("rotate based on position of letter d", b"decab"),
        ];

        let mut word = scrambler::WordScrambler::new(b"abcde");

        for &(rule, result) in rules.iter() {
            word.apply_rule(&rule.parse().unwrap()).unwrap();
            assert_eq!(result, word.word_bytes());
        }
    }

    #[test]
    fn ex2() {
        let rules: [&'static str; 14] = [
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "reverse positions 0 through 4",
            "rotate left 1 step",
            "move position 1 to position 4",
            "move position 3 to position 0",
            "rotate based on position of letter a",
            "rotate based on position of letter b",
            "rotate based on position of letter c",
            "rotate based on position of letter d",
            "rotate based on position of letter e",
            "rotate based on position of letter f",
            "rotate based on position of letter g",
            "rotate based on position of letter h",
        ];

        const WORD: &[u8] = b"abcdefgh";

        let mut scrambler = scrambler::WordScrambler::new(WORD);

        // Ensure that each rule can reverse itself
        for rule in rules.iter() {
            let rule = rule.parse().unwrap();
            scrambler.apply_rule(&rule).unwrap();
            scrambler.reverse_rule(&rule).unwrap();
            assert_eq!(WORD, scrambler.word_bytes());
        }
    }
}
