//! Solution for Advent of Code [2018 Day 03](https://adventofcode.com/2018/day/3).

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

mod fabric {
    use crate::common::util::Pt;
    use std::error::Error;
    use std::fmt;
    use std::str::FromStr;

    const MAX_FABRIC_DIM: usize = 999;

    const MAX_FABRIC_SIZE: usize = MAX_FABRIC_DIM * MAX_FABRIC_DIM;

    /// The integer type used to represent a fabric Claim ID.
    type ClaimId = u16;

    /// The integer type used to store offset on a sheet of fabric.
    type Inch = i16;

    /// The integer type used to count overlapping claims on a particular
    /// cell on a sheet of fabric.
    type CellCounter = u16;

    #[derive(Debug)]
    /// Error type returned when attempting to parse a ``Claim`` from a string.
    pub struct ClaimParseError(&'static str);

    impl Error for ClaimParseError {
        fn description(&self) -> &str { self.0 }
    }

    impl fmt::Display for ClaimParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            f.debug_tuple("ClaimParseError").field(&self.0).finish()
        }
    }

    impl From<::std::num::ParseIntError> for ClaimParseError {
        fn from(_: ::std::num::ParseIntError) -> Self {
            Self("malformed fabric claim: claims contains invalid integer")
        }
    }

    impl From<&'static str> for ClaimParseError {
        fn from(s: &'static str) -> Self { Self(s) }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Claim {
        id: ClaimId,
        corner: Pt<Inch>,
        dim: Pt<Inch>,
    }

    impl Claim {
        pub fn id(&self) -> ClaimId { self.id }
    }

    impl FromStr for Claim {
        type Err = ClaimParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let location_marker = s.find("@")
                .ok_or("malformed fabric claim: missing '@'")?;
            let corner_separator = s.find(",")
                .ok_or("malformed fabric claim: missing ',' separator for corner offset")?;
            let dimension_marker = s.find(":")
                .ok_or("malformed fabric claim: missing ':' marker for claim dimension")?;
            let dimension_separator = s.find("x")
                .ok_or("malformed fabric claim: missing 'x' separator for dimension")?;

            let id = s[1..location_marker - 1].parse()?;
            let corner = Pt {
                x: s[location_marker + 2..corner_separator].parse()?,
                y: s[corner_separator + 1..dimension_marker].parse()?,
            };
            let dim = Pt {
                x: s[dimension_marker + 2..dimension_separator].parse()?,
                y: s[dimension_separator + 1..].parse()?,
            };
            Ok(Claim { id, corner, dim })
        }
    }

    /// A MASSIVE sheet of fabric upon which fabric can be claimed.
    pub struct FabricSheet(Vec<CellCounter>);

    impl FabricSheet {
        pub fn new() -> Self { Self(vec![0; MAX_FABRIC_SIZE]) }

        pub fn apply_claim(&mut self, claim: Claim) {
            for y in claim.corner.y..claim.corner.y + claim.dim.y {
                for x in claim.corner.x..claim.corner.x + claim.dim.x {
                    self.0[y as usize * MAX_FABRIC_DIM + x as usize] += 1
                }
            }
        }

        /// Return the number of cells on this sheet of fabric that are
        /// associated with more than one claim.
        pub fn cells_with_overlapping_claims(&self) -> usize {
            self.0.iter().filter(|&&c| c > 1).count()
        }

        /// Return `true` if the given claim overlaps with another claim
        /// that has been applied to this fabric, otherwise `false`.
        ///
        /// This function assumes that the given `Claim` has already been
        /// applied to this sheet of fabric.
        pub fn check_overlapping_claim(&self, claim: Claim) -> bool {
            for y in claim.corner.y..claim.corner.y + claim.dim.y {
                for x in claim.corner.x..claim.corner.x + claim.dim.x {
                    if self.0[y as usize * MAX_FABRIC_DIM + x as usize] != 1 {
                        return true;
                    }
                }
            }
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_claim() {
            assert_eq!(
                "#1 @ 1,3: 4x4".parse::<Claim>().unwrap(),
                Claim { id: 1, corner: Pt { x: 1, y: 3 }, dim: Pt { x: 4, y: 4 } }
            );
            assert_eq!(
                "#2 @ 3,1: 4x4".parse::<Claim>().unwrap(),
                Claim { id: 2, corner: Pt { x: 3, y: 1 }, dim: Pt { x: 4, y: 4 } },
            );
            assert_eq!(
                "#3 @ 5,5: 2x2".parse::<Claim>().unwrap(),
                Claim { id: 3, corner: Pt { x: 5, y: 5 }, dim: Pt { x: 2, y: 2 } },
            );
            assert_eq!(
                "#468 @ 987,64: 123x4".parse::<Claim>().unwrap(),
                Claim { id: 468, corner: Pt { x: 987, y: 64 }, dim: Pt { x: 123, y: 4 } },
            );
        }
    }
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let claims: Vec<fabric::Claim> = pio::fetch_lines(puzzle)?.into_iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut fabric = fabric::FabricSheet::new();

    for &claim in claims.iter() {
        fabric.apply_claim(claim);
    }

    solve_parts!(
        1 => fabric.cells_with_overlapping_claims(),
        2 => claims.iter()
                .find(|&&c| !fabric.check_overlapping_claim(c))
                .map(|&c| c.id())
                .ok_or("no non-overlapping claims exist")?
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(113576, 825, Pz::new(2018, 3))
    }

    #[test]
    fn ex_both() {
        let claims = [
            "#1 @ 1,3: 4x4".parse().unwrap(),
            "#2 @ 3,1: 4x4".parse().unwrap(),
            "#3 @ 5,5: 2x2".parse().unwrap(),
        ];

        let mut fabric = fabric::FabricSheet::new();
        for &claim in claims.iter() {
            fabric.apply_claim(claim);
        }
        assert_eq!(fabric.cells_with_overlapping_claims(), 4,);
        assert_eq!(
            claims
                .iter()
                .find(|&&c| !fabric.check_overlapping_claim(c))
                .map(|&c| c.id())
                .unwrap(),
            3,
        )
    }
}
