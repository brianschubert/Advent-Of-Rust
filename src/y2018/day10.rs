//! Solution for Advent of Code [2018 Day 10](https://adventofcode.com/2018/day/10).

use crate::common::{
    puzzle,
    util::{Grid, IntoPoint, Pt},
};
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

const GROWTH_TOLERANCE: Scalar = 100;

/// The integral type used to represented `Pt` components.
type Scalar = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// A floating light with a position and velocity.
struct Light {
    pos: Pt<Scalar>,
    vel: Pt<Scalar>,
}

/// A screen on which a sequence of `Light`s can be displayed.
struct LightScreen {
    lights: Vec<Light>,
}

impl Light {
    /// Advance this light by one unit of time, updating its position
    /// according to its velocity.
    fn advance(&mut self) {
        self.pos += self.vel
    }

    /// Rollback this light by one unit of time, updating its position
    /// according to its velocity.
    fn rollback(&mut self) {
        self.pos -= self.vel
    }
}

impl FromStr for Light {
    type Err = &'static str;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        // Segments of `s` that define the position and velocity of the Light.
        let mut pt_segments: [&str; 2] = [""; 2];

        // Gather the two string slices of "< , >" delimited coordinate pairs
        for i in 0..2 {
            let segment_start = s
                .find('<')
                .ok_or("light parse error: missing point beginning")?;
            let segment_end = s[segment_start..]
                .find('>')
                .ok_or("light parse error: missing point terminator")?
                + segment_start;
            pt_segments[i] = &s[segment_start + 1..segment_end];
            s = &s[segment_end + 1..];
        }

        // Map the two coordinate pairs in Pt instances
        let points: Vec<Pt<Scalar>> = pt_segments
            .iter()
            .map(|&s| {
                let parts: Vec<Scalar> = s
                    .split(',')
                    .map(|num| {
                        num.trim()
                            .parse()
                            .map_err(|_| "light parse error: invalid scalar")
                    })
                    .collect::<Result<_, _>>()?;
                match &parts[..] {
                    &[x, y] => Ok(Pt { x, y }),
                    _ => Err("Pt must be defined by exactly two scalars"),
                }
            })
            .collect::<Result<_, _>>()?;

        match &points[..] {
            &[pos, vel] => Ok(Light { pos, vel }),
            _ => Err("light parse error: light must be defined by exactly two points: pos and vel"),
        }
    }
}

impl LightScreen {
    /// Advance all lights on this screen to their next position.
    fn next_frame(&mut self) {
        for light in self.lights.iter_mut() {
            light.advance()
        }
    }

    /// Rollback all lights on this screen to their previous position
    fn rollback_frame(&mut self) {
        for light in self.lights.iter_mut() {
            light.rollback()
        }
    }

    /// Compute the mean distance between all of the lights on this
    /// screen and their centroid.
    fn compute_mean_dist(&self) -> Scalar {
        let count = self.lights.len();
        // Compute the mean
        let centroid = self.lights.iter().fold(
            (0, 0),
            |(acc_x, acc_y),
             &Light {
                 pos: Pt { x, y }, ..
             }| (acc_x + x, acc_y + y),
        );
        let centroid: Pt<i64> = (
            centroid.0 as i64 / count as i64,
            centroid.1 as i64 / count as i64,
        )
            .into_pt();
        self.lights
            .iter()
            .map(|&l| l.pos.into_pt().dist_eucl(centroid))
            .sum::<i64>() as i32
    }

    /// Advance all the lights on this screen to the position at which
    /// the mean distance between the lights on the screen and their centroid
    /// is minimized. Return the index of the resulting frame.
    ///
    /// The search for the most compressed frame will end once the rate at
    /// which the mean distance between the points and their centroid begins
    /// increasing in excess of the given `growth_tolerance`. The screen
    /// will rollback to the point at which the mean-centroid-distance was
    /// previously minimized.
    fn advance_to_most_compressed_frame(&mut self, growth_tolerance: Scalar) -> usize {
        let mut min_mean_dist = Scalar::max_value();
        let mut min_frame = 0;

        let mut curr_mean_dist = self.compute_mean_dist();
        let mut curr_frame: usize = 0;

        // Advance the lights screen until
        loop {
            if (curr_mean_dist - min_mean_dist) > growth_tolerance {
                // The mean distance between the points and their centroid
                // has begun increasing by a significant amount. Rollback
                // this light screen to the most-compressed previous frame
                // and exit.
                let frame_offset = curr_frame - min_frame;
                for _ in 0..frame_offset {
                    self.rollback_frame();
                }
                break;
            }
            if curr_mean_dist < min_mean_dist {
                // The mean distance between the points and their centroid
                // is decreasing. Save the current frame and advance to the
                // next frame.
                min_mean_dist = curr_mean_dist;
                min_frame = curr_frame
            }

            self.next_frame();
            curr_mean_dist = self.compute_mean_dist();
            curr_frame += 1;
        }

        min_frame
    }
}

impl fmt::Display for LightScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let light_positions: Vec<_> = self.lights.iter().map(|&l| l.pos).collect();
        let grid = Grid::from_interior_points(&light_positions[..]);
        let light_positions: HashSet<Pt<Scalar>> = HashSet::from_iter(light_positions.into_iter());
        let s = grid
            .iter()
            .map(|p| {
                if light_positions.contains(&p) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<String>();

        s.as_bytes()
            .chunks(grid.columns())
            .map(|row| writeln!(f, "{}", String::from_utf8_lossy(row)))
            .collect()
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input: Vec<Light> = puzzle::fetch_lines(puzzle)?
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut light_grid = LightScreen { lights: input };

    let min_frame = light_grid.advance_to_most_compressed_frame(GROWTH_TOLERANCE);

    solve_parts!(
        both => {
            (
                // Part 1
                format!(
                    ":: :: :: Frame #{} (mean centroid dist.: {}) :: :: ::\n{}",
                    min_frame,
                    light_grid.compute_mean_dist(),
                    light_grid
                ),
                // Part 2
                min_frame
            )
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[&str] = &[
        "position=< 9,  1> velocity=< 0,  2>",
        "position=< 7,  0> velocity=<-1,  0>",
        "position=< 3, -2> velocity=<-1,  1>",
        "position=< 6, 10> velocity=<-2, -1>",
        "position=< 2, -4> velocity=< 2,  2>",
        "position=<-6, 10> velocity=< 2, -2>",
        "position=< 1,  8> velocity=< 1, -1>",
        "position=< 1,  7> velocity=< 1,  0>",
        "position=<-3, 11> velocity=< 1, -2>",
        "position=< 7,  6> velocity=<-1, -1>",
        "position=<-2,  3> velocity=< 1,  0>",
        "position=<-4,  3> velocity=< 2,  0>",
        "position=<10, -3> velocity=<-1,  1>",
        "position=< 5, 11> velocity=< 1, -2>",
        "position=< 4,  7> velocity=< 0, -1>",
        "position=< 8, -2> velocity=< 0,  1>",
        "position=<15,  0> velocity=<-2,  0>",
        "position=< 1,  6> velocity=< 1,  0>",
        "position=< 8,  9> velocity=< 0, -1>",
        "position=< 3,  3> velocity=<-1,  1>",
        "position=< 0,  5> velocity=< 0, -1>",
        "position=<-2,  2> velocity=< 2,  0>",
        "position=< 5, -2> velocity=< 1,  2>",
        "position=< 1,  4> velocity=< 2,  1>",
        "position=<-2,  7> velocity=< 2, -2>",
        "position=< 3,  6> velocity=<-1, -1>",
        "position=< 5,  0> velocity=< 1,  0>",
        "position=<-6,  0> velocity=< 2,  0>",
        "position=< 5,  9> velocity=< 1, -2>",
        "position=<14,  7> velocity=<-2,  0>",
        "position=<-3,  6> velocity=< 2, -1>",
    ];

    #[test]
    fn parse_light() {
        let lights: Vec<Light> = EXAMPLE_INPUT[..3]
            .iter()
            .map(|&s| s.parse().unwrap())
            .collect();

        assert_eq!(
            lights,
            [
                Light {
                    pos: Pt { x: 9, y: 1 },
                    vel: Pt { x: 0, y: 2 },
                },
                Light {
                    pos: Pt { x: 7, y: 0 },
                    vel: Pt { x: -1, y: 0 },
                },
                Light {
                    pos: Pt { x: 3, y: -2 },
                    vel: Pt { x: -1, y: 1 },
                }
            ],
        );
    }

    #[test]
    fn solution() {
        assert_solution!(
            // "RLEZNRAN"
            ":: :: :: Frame #10240 (mean centroid dist.: 6367) :: :: ::\n\
             #####...#.......######..######..#....#..#####.....##....#....#\n\
             #....#..#.......#............#..##...#..#....#...#..#...##...#\n\
             #....#..#.......#............#..##...#..#....#..#....#..##...#\n\
             #....#..#.......#...........#...#.#..#..#....#..#....#..#.#..#\n\
             #####...#.......#####......#....#.#..#..#####...#....#..#.#..#\n\
             #..#....#.......#.........#.....#..#.#..#..#....######..#..#.#\n\
             #...#...#.......#........#......#..#.#..#...#...#....#..#..#.#\n\
             #...#...#.......#.......#.......#...##..#...#...#....#..#...##\n\
             #....#..#.......#.......#.......#...##..#....#..#....#..#...##\n\
             #....#..######..######..######..#....#..#....#..#....#..#....#\n",
            10240,
            puzzle::Selection::new(2018, 10)
        );
    }

    #[test]
    fn ex_both() {
        let mut light_grid = LightScreen {
            lights: EXAMPLE_INPUT.iter().map(|&s| s.parse().unwrap()).collect(),
        };
        let min_frame = light_grid.advance_to_most_compressed_frame(GROWTH_TOLERANCE);

        assert_eq!(
            "#...#..###\n\
             #...#...#.\n\
             #...#...#.\n\
             #####...#.\n\
             #...#...#.\n\
             #...#...#.\n\
             #...#...#.\n\
             #...#..###\n",
            format!("{}", light_grid),
        );

        assert_eq!(3, min_frame);
    }
}
