//! Solution for 2016 Day 01.

use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};
use crate::common::util::Pt;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    solve_parts! {
        both => {
            let (end, intersect) = walk_blocks(input.trim_end())?;
            (end, intersect.ok_or("Instructions never intersect")?)
        }
    }
}

/// Attempts to perform the "walk" described by the specified string.
///
/// Returns a result tuple containing (0) the final distance from the origin
/// after performing the walk and (1) an optional distance to the first point
/// of intersection that occurred during the walk.
///
/// If the specified walk is malformed, an error string is returned.
fn walk_blocks(instr: &str) -> Result<(i16, Option<i16>), &'static str> {
    let mut pos = Pt::origin();
    let mut dir: Pt<i8> = Pt::n();

    let mut previous = Vec::new();
    let mut intersect: Option<Pt<i16>> = None;

    for instr in instr.split(", ") {
        let (turn, mag) = instr.split_at(1);
        dir = match turn {
            "R" => dir.rot90r(),
            "L" => dir.rot90l(),
            _ => return Err("Malformed turn direction")
        };

        for _ in 0..mag.parse().map_err(|_| "malformed move magnitude")? {
            pos += dir;
            if intersect.is_none() {
                if previous.contains(&pos) {
                    intersect = Some(pos);
                } else {
                    previous.push(pos);
                }
            }
        }
    }

    Ok((
        pos.dist_manh(&Pt::origin()),
        intersect.map(|p| p.dist_manh(&Pt::origin()))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution! {
            332,
            166,
            Pz::new(2016,1)
        }
    }

    #[test]
    fn ex1() {
        assert_eq!(5, walk_blocks("R2, L3").unwrap().0);
        assert_eq!(2, walk_blocks("R2, R2, R2").unwrap().0);
        assert_eq!(12, walk_blocks("R5, L5, R5, R3").unwrap().0);
    }

    #[test]
    fn ex2() {
        assert_eq!(4, walk_blocks("R8, R4, R4, R8").unwrap().1.unwrap())
    }
}
