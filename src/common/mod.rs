pub mod point;
pub mod input;
pub mod solution;

pub use self::point::Pt;
pub use self::solution::{Solution, Answer};
pub use self::input::selector::PuzzleSelection;

pub type BTGrid<T, U> = ::std::collections::BTreeMap<Pt<T>, U>;
pub type HGrid<T, U> = ::std::collections::HashMap<Pt<T>, U>;

macro_rules! route_days {
    ( $( $day:expr => $sol:ident ),+ ) => {
        use common::{Solution, PuzzleSelection as Pz};
        pub fn route(puzzle: Pz) -> Solution {
            match puzzle.day() {
                $( $day => $sol::solve(puzzle), )*
                _ => Solution::empty()
            }
        }
    };
}

macro_rules! bench_ans {
    ( $ans:expr ) => {{
        use common::Answer;
        use std::time::Instant;

        let start = Instant::now();
        Answer::new($ans, Some(Instant::now().duration_since(start)))
    }};
}

macro_rules! solve_parts {
    ( 1 => $part_one:expr ) => { Solution(Some(bench_ans!($part_one)), None) };

    ( 1 => $part_one:expr, 2 => $part_two:expr ) => {
        Solution(
            Some(bench_ans!($part_one)),
            Some(bench_ans!($part_two))
        )
    };

   ( both => $part_producer:expr ) => {{
        use common::Answer;
        use std::time::Instant;

        let start = Instant::now();
        let (part_one, part_two) = $part_producer;
        let bench = start.elapsed();

        Solution(
            Some(Answer::new(part_one, Some(bench))),
            Some(Answer::new(part_two, Some(bench)))
        )
   }}
}

macro_rules! assert_solution {
    ( $part_one:expr, $puzzle:expr) => {{
        use common::{Solution, Answer};
        assert_eq! {
            Solution::new(Answer::from($part_one), None),
            solve($puzzle)
        }
    }};

    ( $part_one:expr, $part_two:expr, $puzzle:expr) => {{
        use common::{Solution, Answer};
        assert_eq! {
            Solution::new(Answer::from($part_one), Answer::from($part_two)),
            solve($puzzle)
        }
    }};
}
