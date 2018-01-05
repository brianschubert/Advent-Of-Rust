// Sub,Mul,Div operators elided until required by a solution
use std::ops::{Add, AddAssign};
use std::cmp::max;

use num_traits::{Signed, NumCast};
use num_traits as nt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
/// A cartesian point on a two-dimensional plane.
///
/// This implementation represents a simplified and expanded rendition of Samuel Cormier-Iijima
/// (@sciyoshi)'s `Pt` structure from his [`advent-of-rust-2017`](https://github.com/sciyoshi/advent-of-rust-2017)
/// crate.
pub struct Pt<T>
    where T: Signed + Ord + NumCast + Copy 
{
    pub x: T,
    pub y: T,
}

impl<T: Signed + Ord + NumCast + Copy> Pt<T> {
    /// Builds a point on the origin: 0,0.
    pub fn origin() -> Self {
        Pt { x: T::zero(), y: T::zero() }
    }

    /// Builds a north offset.
    pub fn n() -> Self {
        Pt { x: T::zero(), y: T::one() }
    }

    /// Builds a north-east offset.
    pub fn ne() -> Self {
        Pt { x: T::one(), y: T::one() }
    }

    /// Builds an east offset.
    pub fn e() -> Self {
        Pt { x: T::one(), y: T::zero() }
    }

    /// Builds a south-east offset.
    pub fn se() -> Self {
        Pt { x: T::one(), y: -T::one() }
    }

    /// Builds a south offset.
    pub fn s() -> Self {
        Pt { x: T::zero(), y: -T::one() }
    }

    /// Builds a south-west offset.
    pub fn sw() -> Self {
        Pt { x: -T::one(), y: -T::one() }
    }

    /// Builds a west offset.
    pub fn w() -> Self {
        Pt { x: -T::one(), y: T::zero() }
    }

    /// Builds a north-west offset.
    pub fn nw() -> Self {
        Pt { x: -T::one(), y: T::one() }
    }

    /// Returns this point rotated right about the origin by 90 degrees.
    ///
    /// Note: takes ownership of self.
    pub fn rot90r(self) -> Self {
        Pt { x: self.y, y: -self.x }
    }

    /// Returns this point rotated left about the origin by 90 degrees.
    ///
    /// Note: takes ownership of self.
    pub fn rot90l(self) -> Self {
        Pt { x: -self.y, y: self.x }
    }

    /// Returns this point's manhattan distance from the specified point.
    pub fn dist_manh(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Returns this point's euclidean distance from the specified point.
    pub fn dist_eucl(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        nt::cast(nt::cast::<_, f64>(dx * dx + dy * dy).unwrap().sqrt()).unwrap()
    }

    /// Returns this point's tile distance from the specified point.
    pub fn dist_tile(&self, other: &Self) -> T {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    /// Returns the four points neighboring this point about the cardinal directions.
    ///
    /// i.e. north, south, east, west.
    pub fn nb_card(&self) -> Vec<Self> {
        vec![
            self + Pt::n(),
            self + Pt::e(),
            self + Pt::s(),
            self + Pt::w()
        ]
    }

    /// Returns the eight points neighboring this point about the ordinal directions.
    ///
    /// e.g. north, north-east, east, south-east, south, etc.
    pub fn nb_ord(&self) -> Vec<Self> {
        vec![
            self + Pt::n(),
            self + Pt::ne(),
            self + Pt::e(),
            self + Pt::se(),
            self + Pt::s(),
            self + Pt::sw(),
            self + Pt::w(),
            self + Pt::nw(),
        ]
    }

    /// Returns a tuple representing the x- and y-components of this point, respectively.
    pub fn parts(&self) -> (T, T) {
        (self.x, self.y)
    }
}

// Heterogeneous point operators

// Add Pt<T> + Pt<U>
//
// For now, we'll require explicit differencing when two
// different flavors of points are used.
impl<T, U> Add<Pt<U>> for Pt<T>
    where T: Signed + Ord + NumCast + Copy,
          U: Signed + Ord + NumCast + Copy
{
    type Output = Pt<T>;

    fn add(self, other: Pt<U>) -> Self::Output {
        Pt {
            x: self.x + nt::cast(other.x).unwrap(),
            y: self.y + nt::cast(other.y).unwrap(),
        }
    }
}

// AddAssign for Pt<T> += Pt<U>
impl<T, U> AddAssign<Pt<U>> for Pt<T>
    where T: Signed + Ord + NumCast + Copy,
          U: Signed + Ord + NumCast + Copy
{
    fn add_assign(&mut self, other: Pt<U>) {
        self.x = self.x + nt::cast(other.x).unwrap();
        self.y = self.y + nt::cast(other.y).unwrap();
    }
}

// Homogeneous point operators

// Add for Pt + &Pt
impl<'a, T> Add<&'a Pt<T>> for Pt<T>
    where T: Signed + Ord + NumCast + Copy 
{
    type Output = Pt<T>;

    fn add(self, other: &'a Pt<T>) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Add &Pt + Pt
impl<'a, T> Add<Pt<T>> for &'a Pt<T>
    where T: Signed + Ord + NumCast + Copy 
{
    type Output = Pt<T>;

    fn add(self, other: Pt<T>) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Add for &Pt + &Pt
impl<'a, T> Add for &'a Pt<T>
    where T: Signed + Ord + NumCast + Copy 
{
    type Output = Pt<T>;

    fn add(self, other: &'a Pt<T>) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right() {
        let mut p: Pt<i8> = Pt::n();

        p = p.rot90r();
        assert_eq!(Pt::e(), p);

        p = p.rot90r();
        assert_eq!(Pt::s(), p);

        p = p.rot90r();
        assert_eq!(Pt::w(), p);

        p = p.rot90r();
        assert_eq!(Pt::n(), p);
    }

    #[test]
    fn rotate_left() {
        let mut p: Pt<i8> = Pt::n();

        p = p.rot90l();
        assert_eq!(Pt::w(), p);

        p = p.rot90l();
        assert_eq!(Pt::s(), p);

        p = p.rot90l();
        assert_eq!(Pt::e(), p);

        p = p.rot90l();
        assert_eq!(Pt::n(), p);
    }

    #[test]
    fn walk_distances() {
        let mut dir: Pt<i8> = Pt::e();
        let mut loc: Pt<i32> = Pt::origin();

        for _ in 0..28 {
            loc = loc + dir;
        }

        dir = dir.rot90r();

        for _ in 0..21 {
            loc = loc + dir;
        }

        assert_eq!(Pt { x: 28, y: -21, }, loc);

        assert_eq!(35, loc.dist_eucl(&Pt::origin()));
        assert_eq!(49, loc.dist_manh(&Pt::origin()));
        assert_eq!(28, loc.dist_tile(&Pt::origin()));
    }
}
