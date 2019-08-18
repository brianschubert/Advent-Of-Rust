use advent_of_rust;

use advent_of_rust::common::puzzle::{Result, Selection, SelectionError, Summary};
use std::error::Error;
use std::io::Write;
use std::{env, io, process, time};

/// Application entry point.
fn main() {
    let puzzle = Selection::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Invalid arguments: {}", err.description());
        eprintln!("Usage: <year:4> <day:2> [input file]");
        process::exit(1);
    });

    let summary = run_solution(puzzle);

    print!("{}", summary);
}

/// Solves the specified puzzle, returning a summary of its solution.
fn run_solution(puzzle: Selection) -> Summary {
    let start = time::Instant::now();
    print!("Solving {:4} day {:02} ... ", puzzle.year(), puzzle.day());
    io::stdout().flush().expect("failed to write to stdout");

    let solution: Result = match puzzle.year() {
        2015 => advent_of_rust::y2015::route(&puzzle),
        2016 => advent_of_rust::y2016::route(&puzzle),
        2018 => advent_of_rust::y2018::route(&puzzle),
        _ => Err(Box::new(SelectionError::UnimplementedYear)),
    };

    println!("{}", if solution.is_ok() { "OK" } else { " FAILED" });

    Summary::new(puzzle, solution, start.elapsed())
}
