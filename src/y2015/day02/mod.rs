//! Solution for 2015 Day 02

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input: Vec<present::Present> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    solve_parts! {
        1 =>  input.iter().map(present::Present::wrapping_paper).sum::<u32>(),
        2 =>  input.iter().map(present::Present::ribbon).sum::<u32>()
    }
}

mod present {
    use std::str::FromStr;

    /// A right-rectangular prism present.
    pub struct Present {
        sides: [u8; 3]
    }

    impl Present {
        /// Returns the amount of wrapping paper required to wrap this
        /// present.
        pub fn wrapping_paper(&self) -> u32 {
            (3 * u32::from(self.sides[0]) * u32::from(self.sides[1]))
                + (2 * u32::from(self.sides[1]) * u32::from(self.sides[2]))
                + (2 * u32::from(self.sides[2]) * u32::from(self.sides[0]))
        }

        /// Returns the amount of ribbon required to wrap this present.
        pub fn ribbon(&self) -> u32 {
            u32::from(2 * (self.sides[0] + self.sides[1]))
                + (u32::from(self.sides[0]) * u32::from(self.sides[1]) * u32::from(self.sides[2]))
        }
    }

    impl FromStr for Present {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut dims: Vec<u8> = s
                .split('x')
                .map(str::parse)
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|_| "malformed dimension")?;

            if 3 == dims.len() {
                dims.sort();
                Ok(Present { sides: [dims[0], dims[1], dims[2]] })
            } else {
                Err("present must have three dimensions")
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
            1_588_178,
            3_783_758,
            Pz::new(2015, 2)
        );
    }

    #[test]
    fn ex_both() {
        let test_cases: [(u8, u8, &'static str); 2] = [
            (58, 34, "2x3x4"),
            (43, 14, "1x1x10"),
        ];

        for &(paper, ribbon, input) in test_cases.iter() {
            let present: present::Present = input.parse().unwrap();
            assert_eq!(u32::from(paper), present.wrapping_paper());
            assert_eq!(u32::from(ribbon), present.ribbon());
        }
    }
}