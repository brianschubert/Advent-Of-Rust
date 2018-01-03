pub mod selector;

use std::fs::File;
use std::io::{Result as IOResult, Read, BufReader, BufRead};

use self::selector::PuzzleSelection as Pz;

pub fn fetch_string(puzzle: Pz) -> IOResult<String> {
    let f = File::open(puzzle.path())?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_to_string(&mut input)?;

    // remove trailing whitespace
    Ok(input.trim_right().to_owned())
}

pub fn fetch_bytes(puzzle: Pz) -> IOResult<Vec<u8>> {
    let f = File::open(puzzle.path())?;
    let mut buf = BufReader::new(f);

    let mut input = Vec::new();
    buf.read_to_end(&mut input)?;

    Ok(input)
}

pub fn fetch_lines(puzzle: Pz) -> IOResult<Vec<String>> {
    let f = File::open(puzzle.path())?;
    let buf = BufReader::new(f);

    let lines: Vec<_> = buf
        .lines()
        .map(|line| line.unwrap())
        .collect();

    Ok(lines)
}
