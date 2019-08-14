//! Solution for 2015 Day 05

use std::collections::HashSet;
use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

/// Vowel bytes.
const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

/// Byte paris that MUST NOT be in "nice" strings.
const FORBIDDEN_PAIRS: [(u8, u8); 4] = [
    (b'a', b'b'), (b'c', b'd'), (b'p', b'q'), (b'x', b'y')
];

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_lines(puzzle)?;

    solve_parts! {
        1 => input.iter().filter(|&s| check_nice_one(s)).count(),
        2 => input.iter().filter(|&s| check_nice_two(s)).count()
    }
}

/// Returns true if the specified string is considered "nice" according
/// to part one's rule set.
fn check_nice_one<S: AsRef<str>>(s: S) -> bool {
    let mut vowel_count = 0_u32;
    let mut contains_double = false;
    let bytes = s.as_ref().as_bytes();

    // Check if first char is vowel
    if bytes.get(0).map(|b| VOWELS.contains(b)).unwrap_or(false) {
        vowel_count += 1;
    }

    for win in bytes.windows(2) {
        if FORBIDDEN_PAIRS.contains(&(win[0], win[1])) {
            return false;
        }
        if VOWELS.contains(&win[1]) {
            vowel_count += 1;
        }
        if win[0] == win[1] {
            contains_double = true;
        }
    }
    contains_double && vowel_count >= 3
}

/// Returns true if the specified string is considered "nice" according
/// to part two's rule set.
fn check_nice_two<S: AsRef<str>>(s: S) -> bool {
    let bytes = s.as_ref().as_bytes();
    check_double_pair(bytes) && bytes.windows(3)
        .any(|triple| triple[0] == triple[2])
}

/// Returns true if the specified byte slice contains a pair of bytes
/// the occurs at least twice without overlapping.
fn check_double_pair(bytes: &[u8]) -> bool {
    let mut previous_pairs: HashSet<(u8, u8)> = HashSet::new();
    let mut skip_next = false;

    // Windows don't allow checking for overlapping while ignoring
    // quadruple bytes (e.g. wwww)
    for i in 0..bytes.len() - 1 {
        if skip_next {
            skip_next = false;
            continue;
        }

        let cap = i + 2;
        let win = &bytes[i..cap];

        if !previous_pairs.insert((win[0], win[1])) {
            return true;
        } else if cap < bytes.len() && bytes[cap] == win[0]
            && bytes[cap] == win[1] { // ignore overlapping pair
            skip_next = true
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            236,
            51,
            Pz::new(2015, 5)
        )
    }

    #[test]
    fn ex1() {
        // Nice
        assert!(check_nice_one("ugknbfddgicrmopn"));
        assert!(check_nice_one("aaa"));
        // Naughty
        assert!(!check_nice_one("jchzalrnumimnmhp"));
        assert!(!check_nice_one("haegwjzuvuyypxyu"));
        assert!(!check_nice_one("dvszwmarrgswjxmb"));
    }

    #[test]
    fn ex2() {
        // Nice
        assert!(check_nice_two("qjhvhtzxzqqjkmpb"));
        assert!(check_nice_two("xxyxx"));
        assert!(check_nice_two("abcdehhfghijklmhhnh"));
        assert!(check_nice_two("rxexcbwhiywwwwnu"));
        // Naughty
        assert!(!check_nice_two("uurcxstgmygtbstg"));
        assert!(!check_nice_two("ieodomkazucvgmuy"));
        assert!(!check_nice_two("abcdehfghhijklmnhh"));
    }

    #[test]
    fn double_pairs() {
        assert!(check_double_pair(b"xyxy"));
        assert!(check_double_pair(b"qwertaakjjhjppaal"));
        assert!(check_double_pair(b"qwertaaakjjhjppaal"));
        assert!(!check_double_pair(b"aaa"));
    }
}