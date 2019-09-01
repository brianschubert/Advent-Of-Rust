//! Handling of two-dimensional points

use std::cmp::max;
use std::ops;

use num_traits as nt;
use num_traits::{NumCast, Signed};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
/// A cartesian point on a two-dimensional plane.
///
/// This implementation was originally inspired by Samuel
/// Cormier-Iijima (@sciyoshi)'s `Pt` structure from his
/// [`advent-of-rust-2017`](https://github.com/sciyoshi/advent-of-rust-2017)
/// crate.
pub struct Pt<T>
where
    T: Signed + Copy,
{
    pub x: T,
    pub y: T,
}

/// A value-to-point conversion trait.
pub trait IntoPoint<T>
where
    T: Signed + Copy,
{
    /// Converts an object into a point.
    fn into_pt(self) -> Pt<T>;
}

impl<T: Signed + Copy> Pt<T> {
    /// Builds a point on the origin: 0,0.
    pub fn origin() -> Self {
        Pt {
            x: T::zero(),
            y: T::zero(),
        }
    }

    /// Builds a north offset.
    pub fn n() -> Self {
        Pt {
            x: T::zero(),
            y: T::one(),
        }
    }

    /// Builds a north-east offset.
    pub fn ne() -> Self {
        Pt {
            x: T::one(),
            y: T::one(),
        }
    }

    /// Builds an east offset.
    pub fn e() -> Self {
        Pt {
            x: T::one(),
            y: T::zero(),
        }
    }

    /// Builds a south-east offset.
    pub fn se() -> Self {
        Pt {
            x: T::one(),
            y: -T::one(),
        }
    }

    /// Builds a south offset.
    pub fn s() -> Self {
        Pt {
            x: T::zero(),
            y: -T::one(),
        }
    }

    /// Builds a south-west offset.
    pub fn sw() -> Self {
        Pt {
            x: -T::one(),
            y: -T::one(),
        }
    }

    /// Builds a west offset.
    pub fn w() -> Self {
        Pt {
            x: -T::one(),
            y: T::zero(),
        }
    }

    /// Builds a north-west offset.
    pub fn nw() -> Self {
        Pt {
            x: -T::one(),
            y: T::one(),
        }
    }

    /// Returns this point rotated right about the origin by 90 degrees.
    ///
    /// Note: takes ownership of self.
    pub fn rot90r(self) -> Self {
        Pt {
            x: self.y,
            y: -self.x,
        }
    }

    /// Returns this point rotated left about the origin by 90 degrees.
    ///
    /// Note: takes ownership of self.
    pub fn rot90l(self) -> Self {
        Pt {
            x: -self.y,
            y: self.x,
        }
    }

    /// Returns this point's manhattan distance from the specified point.
    pub fn dist_manh(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Returns the four points neighboring this point about the cardinal directions.
    ///
    /// i.e. north, south, east, west.
    pub fn nb_card(self) -> Vec<Self> {
        vec![
            self + Pt::n(),
            self + Pt::e(),
            self + Pt::s(),
            self + Pt::w(),
        ]
    }

    /// Returns the eight points neighboring this point about the ordinal directions.
    ///
    /// e.g. north, north-east, east, south-east, south, etc.
    pub fn nb_ord(self) -> Vec<Self> {
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
}

impl<T: Signed + Copy + NumCast> Pt<T> {
    /// Returns this point's euclidean distance from the specified point.
    pub fn dist_eucl(self, other: Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        nt::cast(nt::cast::<_, f64>(dx * dx + dy * dy).unwrap().sqrt()).unwrap()
    }
}

impl<T: Signed + Copy + Ord> Pt<T> {
    /// Returns this point's tile distance from the specified point.
    pub fn dist_tile(self, other: Self) -> T {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

impl<T, U> IntoPoint<U> for Pt<T>
where
    T: Signed + NumCast + Copy,
    U: Signed + NumCast + Copy,
{
    fn into_pt(self) -> Pt<U> {
        Pt {
            x: nt::cast(self.x).unwrap(),
            y: nt::cast(self.y).unwrap(),
        }
    }
}

impl<T> IntoPoint<T> for (T, T)
where
    T: Signed + Copy,
{
    fn into_pt(self) -> Pt<T> {
        Pt {
            x: self.0,
            y: self.1,
        }
    }
}

impl<T> ops::Add<Pt<T>> for Pt<T>
where
    T: Signed + Copy,
{
    type Output = Pt<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::AddAssign<Pt<T>> for Pt<T>
where
    T: Signed + Copy,
{
    fn add_assign(&mut self, rhs: Pt<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> ops::Sub<Pt<T>> for Pt<T>
where
    T: Signed + Copy,
{
    type Output = Pt<T>;

    fn sub(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::SubAssign<Pt<T>> for Pt<T>
where
    T: Signed + Copy,
{
    fn sub_assign(&mut self, rhs: Pt<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
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
            loc += dir.into_pt();
        }

        dir = dir.rot90r();

        for _ in 0..21 {
            loc += dir.into_pt();
        }

        assert_eq!(Pt { x: 28, y: -21 }, loc);

        assert_eq!(35, loc.dist_eucl(Pt::origin()));
        assert_eq!(49, loc.dist_manh(Pt::origin()));
        assert_eq!(28, loc.dist_tile(Pt::origin()));
    }
}
