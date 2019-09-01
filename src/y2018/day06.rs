//! Solution for Advent of Code [2018 Day 06](https://adventofcode.com/2018/day/6).

use crate::common::puzzle;
use crate::common::util::{Grid, GridIter, Pt};
use std::i32;

/// The integral type used to represent point coordinates.
type PointScalar = i32;

/// The point type for this solution.
type Point = Pt<PointScalar>;

/// The integral type used to represent the ID of a point.
type PointLabel = u8;

/// The integral type used to count instances of an event.
type Counter = u32;

/// The maximum total distance that a coordinate can be from all
/// the grid points in order to be considered "safe".
///
/// Criteria for Part 2 of the problem.
const SAFE_DISTANCE_SCORE: PointScalar = 10_000;

#[derive(Debug, Clone, Copy)]
/// A point with a label.
///
/// As it turns out, the label is superfluous when
/// solving the problem, but is still helpful for
/// debugging.
struct NamedPoint {
    label: PointLabel,
    loc: Point,
}

/// A grid of points with logical boundaries based on the
/// most extreme point coordinates.
struct PointGrid {
    grid: Grid<PointScalar>,
    points: Vec<NamedPoint>,
}

/// An iterator over square rings of increasing radius, centered at
/// a point.
///
/// The each ring begins with the top-left most point and continues
/// clockwise.
struct PointRingIter {
    center: Point,
    current_iteration: i32,
}

impl PointGrid {
    /// Constructs a new `PointGrid` with the provided points.
    pub fn new(points: Vec<NamedPoint>) -> Self {
        Self {
            grid: Grid::from_interior_points(&points.iter().map(|&p| p.loc).collect::<Vec<_>>()),
            points,
        }
    }

    /// Returns an iterator over all of the points contained in this grid.
    ///
    /// The returned iterator will yield points "left-to-right" and "bottom-
    /// to-top".
    pub fn iter(&self) -> GridIter<PointScalar> {
        self.grid.iter()
    }

    /// Returns the 1) label of the Point that has the greatest finite
    /// number of coordinates nearest to it and 2) the number of coordinates
    /// that are nearest to that Point.
    ///
    /// Solves Part 1 of the problem.
    pub fn find_most_accessible_point(&self) -> (PointLabel, Counter) {
        // The number of coordinates that each point in this grid `points` vec
        // are closest to. `None` indicates infinitely many points.
        let mut area_counts: Vec<Option<Counter>> = vec![Some(0); self.points.len()];

        for point in self.iter() {
            let closest_pt = self.closest_pt_index(point);
            if self.grid.pt_on_edge(point) {
                area_counts[closest_pt] = None;
            } else {
                area_counts[closest_pt].as_mut().map(|count| *count += 1);
            }
        }

        let (greatest_area_pt, count) = area_counts
            .iter()
            .enumerate()
            .filter_map(|(i, &count)| count.map(|c| (i, c)))
            .max_by_key(|&(_, count)| count)
            .expect("point grid MUST contain at least one point with only finitely many nearest coordinates");
        (self.points[greatest_area_pt].label, count)
    }

    /// Computes the size of the largest region such that for each coordinate
    /// in the region, the sum of the coordinate's distances to each point in
    /// this grid is less than `safe_distance_score`.
    ///
    /// Solves Part 2 of the problem.
    pub fn find_largest_safe_region(&self, safe_distance_score: PointScalar) -> Counter {
        // The point that is nearest to the most coordinates should be inside
        // the region of coordinate with a safe distance from all grid points.
        let starting_point: PointLabel = self.find_most_accessible_point().0;
        let starting_point: Point = self
            .points
            .iter()
            .find(|&&p| p.label == starting_point)
            .unwrap()
            .loc;

        let ring_iter = PointRingIter {
            center: starting_point,
            current_iteration: 0,
        };

        ring_iter
            // Map each ring of points around the starting_point into a
            // vector of all of the distance_scores of points whose scores
            // are within the safe distance.
            .map(|coord_ring: Vec<Point>| {
                coord_ring
                    .into_iter()
                    .map(|coord| self.distance_score(coord))
                    .filter(|&score| score < safe_distance_score)
                    .collect::<Vec<PointScalar>>()
            })
            // Examine larger and larger rings around the starting_point
            // until we find a ring with no point that has an acceptable
            // distance score. All larger rings must only have points that
            // are too far from all of the points on this grid.
            .take_while(|coord_ring| coord_ring.len() > 0)
            // Count the number of coordinates that have safe distances
            .map(|coord_ring| coord_ring.len() as Counter)
            .sum()
    }

    /// Returns the index of the point in `self.points` that is nearest to
    /// the specified point.
    ///
    /// Ties are undefined.
    fn closest_pt_index(&self, pt: Point) -> usize {
        self.points
            .iter()
            .enumerate()
            .min_by_key(|(_, &grid_point)| grid_point.loc.dist_manh(pt))
            .expect("grid MUST HAVE at least one point")
            .0
    }

    /// Computes the sum of the distances between the specified point and each
    /// point on this grid.
    fn distance_score(&self, pt: Point) -> PointScalar {
        self.points
            .iter()
            .map(|&grid_point| grid_point.loc.dist_manh(pt))
            .sum()
    }
}

impl Iterator for PointRingIter {
    type Item = Vec<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration == 0 {
            self.current_iteration += 1;
            return Some(vec![self.center]);
        }
        let top_left = self.center
            + Pt {
                x: -self.current_iteration,
                y: self.current_iteration,
            };
        let diameter = self.current_iteration * 2;

        let offsets: Vec<PointScalar> = (0..diameter).collect();

        let ring_points: Vec<Point> = offsets
            .iter()
            // Top points
            .map(|offset| Pt {
                x: top_left.x + offset,
                y: top_left.y,
            })
            // Right points
            .chain(offsets.iter().map(|offset| Pt {
                x: top_left.x + diameter,
                y: top_left.y - offset,
            }))
            // Bottom points
            .chain(offsets.iter().map(|offset| Pt {
                x: top_left.x + diameter - offset,
                y: top_left.y - diameter,
            }))
            // Left Points
            .chain(offsets.iter().map(|offset| Pt {
                x: top_left.x,
                y: top_left.y - diameter + offset,
            }))
            .collect();

        self.current_iteration += 1;
        Some(ring_points)
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_lines(puzzle)?;
    let points = parse_point_lines(&input)?;
    let grid = PointGrid::new(points);

    solve_parts!(
        1 => grid.find_most_accessible_point().1,
        2 => grid.find_largest_safe_region(SAFE_DISTANCE_SCORE),
    )
}

/// Attempts to parse each of the given lines into a point. Each point is assigned
/// a consecutive ID, starting with `b'A'`.
fn parse_point_lines<S: AsRef<str>>(lines: &[S]) -> Result<Vec<NamedPoint>, &'static str> {
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let line = line.as_ref();
            let split = line
                .find(", ")
                .ok_or("malformed point: missing comma separator")?;
            let loc = Pt {
                x: line[..split]
                    .parse()
                    .map_err(|_| "malformed point: bad integer")?,
                y: line[split + 2..]
                    .parse()
                    .map_err(|_| "malformed point: bad integer")?,
            };
            Ok(NamedPoint {
                label: b'A' + index as u8,
                loc,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_ring_iter_returns_correct_rings() {
        const EXPECTED_RINGS: &[&[Point]] = &[
            &[Point { x: 0, y: 0 }],
            &[
                Point { x: -1, y: 1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: -1 },
                Point { x: 0, y: -1 },
                Point { x: -1, y: -1 },
                Point { x: -1, y: 0 },
            ],
        ];
        let ring_iter = PointRingIter {
            center: Point::origin(),
            current_iteration: 0,
        };

        let rings: Vec<Vec<Point>> = ring_iter.take(2).collect();
        assert_eq!(rings, EXPECTED_RINGS,)
    }

    #[test]
    fn solution() {
        assert_solution!(4011, 46054, puzzle::Selection::new(2018, 6))
    }

    #[test]
    fn ex1() {
        let input = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        let points = parse_point_lines(&input).unwrap();
        let grid = PointGrid::new(points);
        assert_eq!((b'E', 17), grid.find_most_accessible_point());
    }

    #[test]
    fn ex2() {
        let input = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        let points = parse_point_lines(&input).unwrap();
        let grid = PointGrid::new(points);
        assert_eq!(16, grid.find_largest_safe_region(32));
    }
}
