//! # Solution for 2016 Day 18

use common::{input as pio, PuzzleSelection as Pz, Solution};

/// The number of rows to generate for part one.
const ROWS_ONE: usize = 40;

/// The number of rows to generate for part two.
const ROWS_TWO: usize = 400_000;

pub fn solve(puzzle: Pz) -> Solution {
    let input = pio::fetch_string(puzzle)
        .expect("input file could not be read");

    let mut floor: floor::Floor = input.parse().expect("malformed input");

    solve_parts! {
        1 => {
            floor.expand_to(ROWS_ONE);
            floor.safe_count()
        },
        2 => {
            floor.expand_to(ROWS_TWO);
            floor.safe_count()
        }
    }
}

mod floor {
    use std::str::FromStr;

    #[derive(Debug, Eq, PartialEq)]
    /// Set set of tile rows comprising a rectangular floor
    pub struct Floor(Vec<TileRow>);

    #[derive(Debug, Eq, PartialEq)]
    /// A row of tiles on a floor.
    ///
    /// `true` signifies a trapped tile.
    struct TileRow(Vec<bool>);

    impl Floor {
        /// Returns the number of safe tiles on the floor.
        pub fn safe_count(&self) -> u32 {
            self.0.iter().map(|r: &TileRow| {
                r.0.iter().filter(|&&b| !b).count() as u32
            }).sum()
        }

        /// Generates new rows until the floor meets the desired size.
        ///
        /// Panics if the floor contains no rows.
        pub fn expand_to(&mut self, size: usize) {
            while self.0.len() < size {
                let next = self.0.last().unwrap().next_row();
                self.0.push(next);
            }
        }
    }

    impl FromStr for Floor {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Floor(vec![s.parse()?]))
        }
    }

    impl TileRow {
        /// Returns the next row of tiles on the floor.
        pub fn next_row(&self) -> Self {
            let mut out = Vec::with_capacity(self.0.len());

            out.push(self.0[1]);

            for win in self.0.windows(3) {
                out.push(win[0] != win[2]);
            }

            out.push(self.0[self.0.len() - 2]);

            TileRow(out)
        }
    }

    impl FromStr for TileRow {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(TileRow(s
                .as_bytes()
                .iter()
                .map(|&b| b == b'^')
                .collect::<Vec<bool>>()
                .into()
            ))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_tilerow() {
            assert_eq!(TileRow(vec![true; 5]), "^^^^^".parse().unwrap());
            assert_eq!(TileRow(vec![false; 5]), ".....".parse().unwrap());
            assert_eq!(
                TileRow(vec![true, false, false, true, false, true, true]),
                "^..^.^^".parse().unwrap()
            )
        }

        #[test]
        fn generate_floor() {
            let mut floor: Floor = "..^^.".parse().unwrap();
            floor.expand_to(3);

            assert_eq!(
                Floor(vec![
                    "..^^.".parse().unwrap(),
                    ".^^^^".parse().unwrap(),
                    "^^..^".parse().unwrap()
                ]),
                floor
            );

            assert_eq!(6, floor.safe_count());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            1978,
            20003246,
            Pz::of(2016, 18)
        )
    }

    #[test]
    fn ex1() {
        let mut floor: floor::Floor = ".^^.^.^^^^".parse().unwrap();
        floor.expand_to(10);
        assert_eq!(38, floor.safe_count());
    }
}
