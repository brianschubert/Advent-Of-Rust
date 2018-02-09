//! Solution for 2016 Day 14
use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::collections::HashMap;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_bytes(puzzle)
        .expect("input file could not be read");
    let input = &input[..input.len() - 1];

    solve_parts! {
        1 => generate_pad_keys(&input, 1).get(63).unwrap(),
        2 => generate_pad_keys(&input, 1 + 2016).get(63).unwrap()
    }
}

// Index of a valid generated one-time pad key
type KeyIndex = u16;

/// Returns the first 64 generator indexes that produce valid
/// one-time pad keys.
fn generate_pad_keys(seed: &[u8], stretch_factor: u16) -> Vec<KeyIndex> {
    let mut out = Vec::with_capacity(64);
    let mut round: KeyIndex = 0;
    let mut keys_found = 0_u8;

    // Could swap with BTree Map for ordered keys
    // Doing so would remove the need to sort the generated keys
    let mut possible_keys: HashMap<KeyIndex, u8> = HashMap::new();

    while keys_found < 64 {
        let next_key = {
            let mut input = Vec::from(seed);
            input.extend(round.to_string().as_bytes());

            md5_stretch(&input[..], stretch_factor)
        };

        // Check for quintuple byte
        if let Some(rep_byte) = find_repeating_byte(&next_key[..], 5) {
            let age_limit = if round < 1000 { 0 } else { round - 1000 };

            out.extend(possible_keys
                .iter()
                .filter(|&(&gen_index, &rep)| {
                    rep == rep_byte && gen_index >= age_limit
                })
                .map(|(&gen_index, _)| {
                    keys_found += 1;
                    gen_index
                })
            );

            possible_keys = possible_keys
                .into_iter()
                .filter(|&(gen_index, rep)| {
                    !(rep == rep_byte) && gen_index >= age_limit
                })
                .collect()
        }

        // Check for triple byte
        if let Some(rep_byte) = find_repeating_byte(&next_key[..], 3) {
            possible_keys.insert(round, rep_byte);
        }

        round += 1;
    }

    out.sort();
    out
}

/// Returns the first byte in the specified string slice that repeats
/// `rep_count` times in a row.
///
/// If no repeating byte is found, None is returned.
fn find_repeating_byte(s: &str, rep_count: usize) -> Option<u8> {
    if rep_count < 2 { return None; }
    for win in s.as_bytes().windows(rep_count) {
        let first = win.first().unwrap();
        if !win.iter().any(|&b| b != *first) {
            return Some(*first);
        }
    }
    None
}

/// Repeatedly computes the MD5 digest of the string according to the
/// specified stretch factor.
fn md5_stretch(s: &[u8], stretch_factor: u16) -> String {
    let mut hasher = Md5::new();
    let mut result = String::new();

    hasher.input(&s);

    for _ in 0..stretch_factor {
        hasher.input_str(&result[..]);
        result = hasher.result_str();
        hasher.reset();
    }

    result
}

#[cfg(test)]
// Tests associated with part two are ignored by default as they
// take a few minutes to run (w-wooo! key stretching...).
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn solution() {
        assert_solution! {
            23890,
            22696,
            Pz::new(2016, 14)
        }
    }

    #[test]
    fn ex1() {
        let keys = generate_pad_keys(b"abc", 1);

        assert_eq!(22728, *keys.get(63).unwrap());
    }

    #[test]
    #[ignore]
    fn ex2() {
        let keys = generate_pad_keys(b"abc", 1 + 2016);

        assert_eq!(22551, *keys.get(63).unwrap())
    }

    #[test]
    fn find_byte_repeats() {
        // Edge cases
        assert_eq!(None, find_repeating_byte("12345", 0));
        assert_eq!(None, find_repeating_byte("12345", 1));

        // Find repetitions
        assert_eq!(None, find_repeating_byte("qwertyuop###jghfgd", 4));
        assert_eq!(Some(b'#'), find_repeating_byte("qwertyuop###jghfgd", 3));
        assert_eq!(Some(b'#'), find_repeating_byte("qwertyuop###jghfgd", 2));
        assert_eq!(Some(b'3'), find_repeating_byte("387bcfdee8333abcabc", 3));
        assert_eq!(Some(b'3'), find_repeating_byte("387bcfdee833333abcabc", 5));
        assert_eq!(Some(b'3'), find_repeating_byte("387bcfdee833333", 5));
        assert_eq!(Some(b'd'), find_repeating_byte("ddd7bcfdeeigejgiej", 3));

        // Too short
        assert_eq!(None, find_repeating_byte("33", 3));
        assert_eq!(None, find_repeating_byte("", 3));
    }

    #[test]
    fn stretch_md5() {
        const INPUT: &'static [u8] = b"abc0";

        assert_eq!("577571be4de9dcce85a041ba0410f29f", md5_stretch(&INPUT, 1));
        assert_eq!("eec80a0c92dc8a0777c619d9bb51e910", md5_stretch(&INPUT, 2));
        assert_eq!("16062ce768787384c81fe17a7a60c7e3", md5_stretch(&INPUT, 3));
        assert_eq!("a107ff634856bb300138cac6568c0f24", md5_stretch(&INPUT, 2017));
    }
}
