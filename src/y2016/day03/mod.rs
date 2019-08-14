//! Solution for 2016 Day 03.

use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

#[derive(Debug)]
struct Triangle(u16, u16, u16);

impl Triangle {
    fn is_valid(&self) -> bool {
        let Triangle(a, b, c) = *self;
        if a > b && a > c {
            b + c > a
        } else if b > c {
            a + c > b
        } else {
            a + b > c
        }
    }
}

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let tri_desc: Vec<Vec<u16>> = pio::fetch_lines(puzzle)?
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse())
                .collect()
        }).collect::<Result<_, _>>()?;

    solve_parts! {
        1 => triangles_by_row(&tri_desc).into_iter().filter(|tri| tri.is_valid()).count(),
        2 => triangles_by_col(&tri_desc).into_iter().filter(|tri| tri.is_valid()).count()
    }
}

fn triangles_by_row(tri_desc: &[Vec<u16>]) -> Vec<Triangle> {
    tri_desc.iter().map(|sides| {
        debug_assert!(sides.len() == 3);
        Triangle(sides[0], sides[1], sides[2])
    }).collect()
}

fn triangles_by_col(tri_desc: &[Vec<u16>]) -> Vec<Triangle> {
    tri_desc.chunks(3).flat_map(|three_tri| {
        three_tri[0].iter()
            .zip(three_tri[1].iter())
            .zip(three_tri[2].iter())
            .map(|col| {
                let ((a, b), c) = col;
                Triangle(*a, *b, *c)
            })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            983,
            1836,
            Pz::new(2016,3)
        );
    }
}
