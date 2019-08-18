//! Functions for collecting puzzle input.

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result as IOResult};

use super::selector::Selection as Pz;

pub fn fetch_string(puzzle: &Pz) -> IOResult<String> {
    let f = File::open(puzzle.path())?;
    let mut buf = BufReader::new(f);

    let mut input = String::new();
    buf.read_to_string(&mut input)?;

    Ok(input)
}

#[deprecated]
// Deprecated in favor of reading directly to a string. Input files may
// include trailing whitespace, which may be handled succinctly with
// str methods.
pub fn fetch_bytes(puzzle: &Pz) -> IOResult<Vec<u8>> {
    let f = File::open(puzzle.path())?;
    let mut buf = BufReader::new(f);

    let mut input = Vec::new();
    buf.read_to_end(&mut input)?;

    Ok(input)
}

pub fn fetch_lines(puzzle: &Pz) -> IOResult<Vec<String>> {
    let f = File::open(puzzle.path())?;
    let buf = BufReader::new(f);

    let lines: Result<Vec<_>, _> = buf.lines().collect();

    Ok(lines?)
}
