// Sub,Mul,Div operators elided until required by a solution
use std::ops::{Add, AddAssign};
use std::cmp::max;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
/// A cartesian point on a two-dimensional plane.
///
/// This implementation represents a simplified and expanded rendition of Samuel Cormier-Iijima
/// (@sciyoshi)'s `Pt` structure from his [`advent-of-rust-2017`](https://github.com/sciyoshi/advent-of-rust-2017)
/// crate.
pub struct Pt {
    pub x: i64,
    pub y: i64,
}

impl Pt {
    /// Builds a point on the origin: 0,0.
    pub fn origin() -> Pt {
        Pt { x: 0, y: 0 }
    }

    /// Builds a north offset.
    pub fn n() -> Pt {
        Pt { x: 0, y: 1 }
    }

    /// Builds a north-east offset.
    pub fn ne() -> Pt {
        Pt { x: 1, y: 1 }
    }

    /// Builds an east offset.
    pub fn e() -> Pt {
        Pt { x: 1, y: 0 }
    }

    /// Builds a south-east offset.
    pub fn se() -> Pt {
        Pt { x: 1, y: -1 }
    }

    /// Builds a south offset.
    pub fn s() -> Pt {
        Pt { x: 0, y: -1 }
    }

    /// Builds a south-west offset.
    pub fn sw() -> Pt {
        Pt { x: -1, y: -1 }
    }

    /// Builds a west offset.
    pub fn w() -> Pt {
        Pt { x: -1, y: 0 }
    }

    /// Builds a north-west offset.
    pub fn nw() -> Pt {
        Pt { x: -1, y: 1 }
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
    pub fn dist_manh(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Returns this point's euclidean distance from the specified point.
    pub fn dist_eucl(&self, other: &Self) -> i64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt() as i64
    }

    /// Returns this point's tile distance from the specified point.
    pub fn dist_tile(&self, other: &Self) -> i64 {
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
    pub fn parts(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

// Add Pt + Pt
impl Add for Pt {
    type Output = Pt;

    fn add(self, other: Self) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add for Pt + &Pt
impl<'a> Add<&'a Pt> for Pt {
    type Output = Pt;

    fn add(self, other: &'a Pt) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Add &Pt + Pt
impl<'a> Add<Pt> for &'a Pt {
    type Output = Pt;

    fn add(self, other: Pt) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Add for &Pt + &Pt
impl<'a> Add for &'a Pt {
    type Output = Pt;

    fn add(self, other: &'a Pt) -> Self::Output {
        Pt {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// AddAssign for Pt += Pt
impl AddAssign for Pt {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right() {
        let mut p = Pt::n();

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
        let mut p = Pt::n();

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
        let mut dir = Pt::e();
        let mut loc = Pt::origin();

        for _ in 0..28 {
            loc = loc + dir;
        }

        dir = dir.rot90r();

        for _ in 0..21 {
            loc += dir;
        }

        assert_eq!(Pt { x: 28, y: -21, }, loc);

        assert_eq!(35, loc.dist_eucl(&Pt::origin()));
        assert_eq!(49, loc.dist_manh(&Pt::origin()));
        assert_eq!(28, loc.dist_tile(&Pt::origin()));
    }
}
