//! Common structures for handling regions of points
//!
//! Used in 2018 Days 6 and 10.

use super::Pt;
use num_traits::{self as nt, NumCast, Signed};

#[derive(Debug, Clone)]
/// A rectangular section of points in space.
pub struct Grid<T>
where
    T: Signed + Copy + Ord,
{
    bottom_left: Pt<T>,
    top_right: Pt<T>,
}

/// An iterator over a rectangular grid of points.
pub struct GridIter<'a, T>
where
    T: Signed + Copy + Ord,
{
    grid: &'a Grid<T>,
    next: Option<Pt<T>>,
}

impl<T> Grid<T>
where
    T: Signed + Copy + Ord,
{
    /// Constructs a rectangular grid of points that contains all of the
    /// given points.
    pub fn from_interior_points(points: &[Pt<T>]) -> Self
    where
        T: Signed + Copy + Ord,
    {
        if points.is_empty() {
            panic!("grid must be defined by at least one point")
        }
        let mut bottom_left = points[0];
        let mut top_right = points[0];

        for &point in &points[1..] {
            if point.x < bottom_left.x {
                bottom_left.x = point.x
            } else if point.x > top_right.x {
                top_right.x = point.x
            }
            if point.y < bottom_left.y {
                bottom_left.y = point.y
            } else if point.y > top_right.y {
                top_right.y = point.y
            }
        }

        Self {
            bottom_left,
            top_right,
        }
    }

    /// Creates grid with the given corner points.
    ///
    /// Panics if any component of the `bottom_left` point is greater than
    /// the corresponding component of the `top_right` point.
    pub fn from_corners(bottom_left: Pt<T>, top_right: Pt<T>) -> Self {
        if bottom_left.x > top_right.x || bottom_left.y > top_right.y {
            panic!("bottom-left corner of a grid MUST have components less than or equal to those of the top-right corner")
        }
        Grid {
            bottom_left,
            top_right,
        }
    }

    #[inline(always)]
    /// Returns the bottom-left most point in this grid.
    pub fn bottom_left(&self) -> Pt<T> {
        self.bottom_left
    }

    #[inline(always)]
    /// Returns the top-right most point in this grid.
    pub fn top_right(&self) -> Pt<T> {
        self.top_right
    }

    /// Returns an iterator over all of the points contained in this grid.
    ///
    /// The returned iterator will yield points "left-to-right" and "bottom-
    /// to-top".
    ///
    /// # Example
    /// ```
    /// # use advent_of_rust::common::util::{Pt, Grid};
    /// let grid = Grid::from_interior_points(&[Pt::origin(), Pt::ne()]);
    /// let contained_points: Vec<Pt<i8>> = grid.iter().collect();
    ///
    /// assert_eq!(
    ///     contained_points,
    ///     &[Pt::origin(), Pt::e(), Pt::n(), Pt::ne()]
    /// );
    /// ```
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            next: Some(self.bottom_left),
        }
    }

    /// Returns `true` if the specified point is contained on the edge of
    /// this grid.
    pub fn pt_on_edge(&self, pt: Pt<T>) -> bool {
        pt.x == self.bottom_left.x
            || pt.x == self.top_right.x
            || pt.y == self.bottom_left.y
            || pt.y == self.top_right.y
    }

    /// Returns `true` if the specified point in contained on or within
    /// the boundaries of this grid.
    pub fn contains(&self, pt: Pt<T>) -> bool {
        pt.x >= self.bottom_left.x
            && pt.x <= self.top_right.x
            && pt.y >= self.bottom_left.y
            && pt.y <= self.top_right.y
    }
}

impl<T> Grid<T>
where
    T: Signed + Copy + Ord + NumCast,
{
    /// Returns the number of rows contained in this grid.
    pub fn rows(&self) -> usize {
        nt::cast((self.top_right.y - self.bottom_left.y).abs() + T::one()).unwrap()
    }

    /// Returns the number of columns contained in this grid.
    pub fn columns(&self) -> usize {
        nt::cast((self.top_right.x - self.bottom_left.x).abs() + T::one()).unwrap()
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.columns(), self.rows())
    }
}

impl<T> Iterator for GridIter<'_, T>
where
    T: Signed + Copy + Ord,
{
    type Item = Pt<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|point| {
            self.next = if point.x < self.grid.top_right.x {
                // point is in the middle of a row
                Some(Pt {
                    x: point.x + T::one(),
                    ..point
                })
            } else if point.y < self.grid.top_right.y {
                // point is at the end of a row that is not the top row
                Some(Pt {
                    x: self.grid.bottom_left.x,
                    y: point.y + T::one(),
                })
            } else {
                // point is at the top_right point; there is no next point
                None
            };
            point
        })
    }
}
