//! Functions and structures with common application in puzzle
//! solutions.

pub use self::point::{
    grid::{Grid, GridIter},
    IntoPoint, Pt,
};
pub use self::rotate::RotateSigned;

pub type BTGrid<T, U> = ::std::collections::BTreeMap<Pt<T>, U>;
pub type HGrid<T, U> = ::std::collections::HashMap<Pt<T>, U>;

mod point;
mod rotate;
