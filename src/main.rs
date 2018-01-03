extern crate advent_of_rust;

use std::{env, process};

use advent_of_rust::common::PuzzleSelection;

fn main() {
    let puzzle = PuzzleSelection::new(env::args()).unwrap_or_else(|err_msg| {
        eprintln!("Bad arguments: {}", err_msg);
        eprintln!("Usage: <year:4> <day:2>");
        process::exit(1);
    });

    match puzzle.year() {
        2016 => advent_of_rust::y2016::route(puzzle),
        other => panic!(format!("No solution found for year {}", other))
    }
}
