extern crate advent_of_rust;

use std::{env, process};

use advent_of_rust::common::{PuzzleSelection, Solution, Answer};

fn main() {
    let puzzle = PuzzleSelection::new(env::args()).unwrap_or_else(|err_msg| {
        eprintln!("Bad arguments: {}", err_msg);
        eprintln!("Usage: <year:4> <day:2>");
        process::exit(1);
    });

    let Solution(part_one, part_two) = match puzzle.year() {
        2016 => advent_of_rust::y2016::route(puzzle),
        _ => Solution::empty()
    };

    print_part(1, part_one);
    print_part(2, part_two);
}

fn print_part(part: u8, ans: Option<Answer>) {
    debug_assert!(part == 1 || part == 2, "Bad puzzle part; must be either 1 or 2");

    let out = ans.map_or("Not implemented".to_owned(), |part| {
        part.bench.map_or(format!("`{}` [x]", part.ans), |bench| {
            format!(
                "`{}` [{}.{:09}s]",
                part.ans,
                bench.as_secs(),
                bench.subsec_nanos()
            )
        })
    });

    println!("Part {}: {}", part, out);
}
