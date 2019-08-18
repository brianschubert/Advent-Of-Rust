//! Solution for 2016 Day 16

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

/// Length of data to generate for part one.
const DATA_LENGTH_ONE: usize = 272;

/// Length of data to generate for part two.
const DATA_LENGTH_TWO: usize = 35_651_584;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    let input: Vec<bool> = input
        .trim_end()
        .bytes()
        .map(|b| b == b'1')
        .collect();

    let mut filler = diskfiller::DiskFiller::new();

    solve_parts! {
        1 => {
            filler.input(&input);
            filler.generate_to_length(DATA_LENGTH_ONE);

            let checksum = filler.checksum();
            filler.reset();
            checksum
        },
        2 => {
            filler.input(&input);
            filler.generate_to_length(DATA_LENGTH_TWO);

            let checksum = filler.checksum();
            filler.reset();
            checksum
        }

    }
}

mod diskfiller {
    /// Generator of data to fill a disk.
    ///
    /// Data (unfortunately) stored as a `Vec` of bools  - despite the
    /// horrendous memory inefficiency - for ease of implementation.
    pub struct DiskFiller {
        data: Vec<bool>,
        cut: usize,
    }

    impl DiskFiller {
        /// Creates a new DiskFiller.
        pub fn new() -> Self {
            DiskFiller { data: Vec::new(), cut: 0 }
        }

        /// Appends the specified data to this disk filler's data.
        pub fn input(&mut self, data: &[bool]) {
            self.data.extend_from_slice(&data);
        }

        /// Generates data of at least the specified length based
        /// the data previously provided.
        pub fn generate_to_length(&mut self, length: usize) {
            if length <= self.data.len() {
                return;
            }

            self.cut = length;

            while self.data.len() < self.cut {
                let mut mirror = self.data.clone();
                mirror.reverse();

                self.data.push(false);

                self.data.extend(mirror.into_iter().map(|b| !b));
            }
        }

        #[cfg(test)]
        /// Returns this disk filler's data as a binary string.
        ///
        /// Needed only for unit tests.
        pub fn result_str(&self) -> String {
            stringify_bool_slice(&self.data[..self.cut])
        }

        /// Computes the checksum of this filler's data.
        pub fn checksum(&self) -> String {
            let mut result = Vec::from(&self.data[..self.cut]);

            while {
                let len = result.len();
                len == self.cut || len & 1 == 0
            } {
                result = result
                    .chunks(2)
                    .map(|pair| pair[0] == pair[1])
                    .collect();
            }

            stringify_bool_slice(&result[..])
        }

        /// Resets this disk filler's internal state.
        pub fn reset(&mut self) {
            self.data.clear();
            self.cut = 0;
        }
    }

    /// Converts a slice of bools into a binary string.
    fn stringify_bool_slice(data: &[bool]) -> String {
        data.iter()
            .map(|&b| if b { '1' } else { '0' })
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn generate_filler_data() {
            let test_cases: [(&'static [bool], &'static str); 4] = [
                (&[true], "100"),
                (&[false], "001"),
                (&[true, true, true, true, true], "11111000000"),
                (
                    &[
                        true, true, true, true,
                        false, false, false, false,
                        true, false, true, false
                    ],
                    "1111000010100101011110000",
                ),
            ];

            let mut filler = DiskFiller::new();

            for &(input, expected) in test_cases.iter() {
                filler.input(&input);
                filler.generate_to_length(expected.len());

                assert_eq!(expected, filler.result_str());
                filler.reset();
            }
        }

        #[test]
        fn filler_checksum() {
            let filler = DiskFiller {
                data: b"110010110100"
                    .iter()
                    .map(|&b| b == b'1')
                    .collect(),
                cut: 12,
            };

            assert_eq!("100", filler.checksum());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!("11111000111110000", "10111100110110100", Pz::new(2016, 16))
    }

    #[test]
    fn ex1() {
        let mut filler = diskfiller::DiskFiller::new();

        filler.input(&b"10000".iter().map(|&b| b == b'1').collect::<Vec<_>>()[..]);

        filler.generate_to_length(20);

        assert_eq!("10000011110010000111", filler.result_str());

        assert_eq!("01100", filler.checksum());
    }
}
