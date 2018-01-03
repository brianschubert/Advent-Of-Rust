pub mod point;
pub mod input;

pub use self::point::Pt;
pub use self::input::selector::PuzzleSelection;

pub type BTGrid<T> = ::std::collections::BTreeMap<Pt, T>;
pub type HGrid<T> = ::std::collections::HashMap<Pt, T>;

#[macro_export]
macro_rules! route_days {
    ( $( $day:expr => $sol:ident ),+ ) => {
        use common::PuzzleSelection as Pz;
        pub fn route(puzzle: Pz) {
            match puzzle.day() {
                $( $day => $sol::solve(puzzle), )*
                other => panic!(format!("No solution found for day {}", other))
            }
        }
    };
}
