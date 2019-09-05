//! Solution for Advent of Code [2018 Day 11](https://adventofcode.com/2018/day/11).

use crate::common::puzzle;
use crate::common::util::{Grid, IntoPoint, Pt};
use std::cmp;
use std::ops::Index;

/// Integral type used to represent a component of a point
/// on a power grid.
type CellIndex = i16;

/// `Pt` type used to represent a coordinate on a power grid.
type CellCoordinate = Pt<CellIndex>;

/// Integral type used to represent a signed "power" magnitude.
type Power = i32;

/// A grid of power cells.
struct PowerGrid {
    grid: Grid<CellIndex>,
    cells: Vec<Power>,
}

impl PowerGrid {
    /// Creates a new power grid with the given grid's dimensions using the
    /// given serial number to compute power cells levels.
    fn generate_new(grid: Grid<CellIndex>, serial_number: Power) -> Self {
        let (rows, columns) = grid.dim();
        let bottom_left = grid.bottom_left();
        let mut cells: Vec<Power> = vec![0; rows * columns];

        for (i, cell) in cells.iter_mut().enumerate() {
            let rack_id = ((i + bottom_left.x as usize) % columns + 10) as Power;
            let mut power_level =
                rack_id * ((i / columns) + bottom_left.y as usize) as Power + serial_number;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            *cell = power_level - 5;
        }

        Self { grid, cells }
    }

    /// Locates the 3x3 square on this grid with the greatest total power
    /// across its cells, if such a square exists.
    ///
    /// Solves Part 1.
    fn find_most_powerful_3_square(&self) -> Result<CellCoordinate, &'static str> {
        let search_grid = Grid::from_interior_points(&[
            self.grid.bottom_left() + Pt::ne(),
            self.grid.top_right() + Pt::sw(),
        ]);

        search_grid
            .iter()
            .map(|center| (center, self.square_power_total(center)))
            .max_by_key(|&(_, power)| power)
            .map(|(coord, _)| coord)
            .ok_or("no 3x3 squares exist on the grid")
    }

    /// Computes the total power of the 3x3 square centered at `center_pt` on
    /// this grid.
    ///
    /// For Part 1.
    fn square_power_total(&self, center_pt: CellCoordinate) -> Power {
        self[center_pt]
            + center_pt
                .nb_ord()
                .into_iter()
                .map(|neighbor| self[neighbor])
                .sum::<Power>()
    }

    /// Locates the square on this grid with the greatest total power across
    /// its cells. Returns 1) the bottom-left coordinate of the found square,
    /// and 2) the dimension of the square.
    ///
    /// This function is a brute-force approach with O(x^3) time
    /// complexity based on the grid's dimension. Calling it is not for
    /// the faint of heart.
    ///
    /// Roughly 8.099e10 additions are required for a 300x300 grid according to
    /// a quick heuristic (sum^{300}_{i=0} {(300-i)^2 * i^2}).
    ///
    /// For Part 2.
    fn find_most_powerful_unbounded_score_brute_force(&self) -> (CellCoordinate, CellIndex) {
        let mut max_power = Power::min_value();
        let mut max_power_corner = self.grid.bottom_left();
        let mut max_power_dim = 0;

        for corner in self.grid.iter() {
            let (corner_max, corner_max_dim) = self.corner_compute_most_powerful_square(corner);
            if corner_max > max_power {
                max_power = corner_max;
                max_power_corner = corner;
                max_power_dim = corner_max_dim;
            }
        }

        (max_power_corner, max_power_dim)
    }

    /// Computes the square fixed on given corner on this grid that has the
    /// greatest total power across its cells. Returns 1) the total power
    /// of the found square, and 2) the the dimension of the square.
    ///
    /// For Part 2.
    fn corner_compute_most_powerful_square(
        &self,
        bottom_left_corner: CellCoordinate,
    ) -> (Power, CellIndex) {
        // Compute the largest possible dimension for a square with the
        // given bottom-right corner located on this grid.
        let top_right = self.grid.top_right();
        let dim_bound = cmp::min(
            top_right.x - bottom_left_corner.x,
            top_right.y - bottom_left_corner.y,
        ) as CellIndex;

        let mut max_power = Power::min_value();
        let mut max_power_dim = 0;

        for dim in 1..=dim_bound {
            let partial_square = Grid::from_corners(
                bottom_left_corner,
                bottom_left_corner + (dim, dim).into_pt(),
            );
            let total_power = partial_square.iter().map(|pt| self[pt]).sum::<Power>();
            if total_power > max_power {
                max_power = total_power;
                max_power_dim = dim;
            }
        }
        (max_power, max_power_dim + 1)
    }
}

impl Index<CellCoordinate> for PowerGrid {
    type Output = Power;

    fn index(&self, index: CellCoordinate) -> &Self::Output {
        if !self.grid.contains(index) {
            panic!("index must be within the boundaries of the grid");
        }
        let bottom_left = self.grid.bottom_left();

        let offset = self.grid.columns() * (index.y - bottom_left.y) as usize
            + (index.x - bottom_left.x) as usize;
        &self.cells[offset]
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input: Power = puzzle::fetch_string(puzzle)?.trim_end().parse()?;

    let power_grid = PowerGrid::generate_new(
        Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
        input,
    );

    solve_parts!(
        1 => {
            let corner = power_grid.find_most_powerful_3_square()? + Pt::sw();
            format!("{:?}", corner)
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            format!("{:?}", Pt { x: 243, y: 34 }),
            puzzle::Selection::new(2018, 11),
        )
    }

    #[test]
    fn generate_grid() {
        let test_cases: &[(CellCoordinate, Power, Power)] = &[
            ((3, 5).into_pt(), 8, 4),
            ((122, 79).into_pt(), 57, -5),
            ((217, 196).into_pt(), 39, 0),
            ((101, 153).into_pt(), 71, 4),
        ];

        for &(cell, serial_number, expected_power) in test_cases {
            let power_grid = PowerGrid::generate_new(
                Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
                serial_number,
            );

            assert_eq!(expected_power, power_grid[cell]);
        }
    }

    #[test]
    fn square_power_total() {
        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            18,
        );
        assert_eq!(29, power_grid.square_power_total((34, 46).into_pt()));

        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            42,
        );
        assert_eq!(30, power_grid.square_power_total((22, 62).into_pt()));
    }

    #[test]
    fn corner_compute_most_powerful_square() {
        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            18,
        );

        assert_eq!(
            (113, 16),
            power_grid.corner_compute_most_powerful_square((90, 269).into_pt()),
        );
    }

    #[test]
    fn ex1() {
        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            18,
        );

        assert_eq!(
            (33, 45).into_pt() + Pt::ne(),
            power_grid.find_most_powerful_3_square().unwrap()
        );

        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            42,
        );
        assert_eq!(
            (21, 61).into_pt() + Pt::ne(),
            power_grid.find_most_powerful_3_square().unwrap()
        );
    }

    #[test]
    #[ignore]
    fn ex2() {
        let power_grid = PowerGrid::generate_new(
            Grid::from_corners((1, 1).into_pt(), (300, 300).into_pt()),
            18,
        );

        assert_eq!(
            ((90, 269).into_pt(), 16),
            power_grid.find_most_powerful_unbounded_score_brute_force(),
        );
    }
}
