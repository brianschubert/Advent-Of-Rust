//! Solution for 2016 Day 07.

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_lines(puzzle)
        .expect("input file could not be read");

    solve_parts! {
        1 => input.iter().filter(supports_snooping).count(),
        2 => input.iter().filter(supports_listening).count()
   }
}

/// Checks if an IPv7 string supports "transport-layer snooping"
fn supports_snooping<S: AsRef<str>>(ipv7: &S) -> bool {
    let byte_wins = ipv7.as_ref().as_bytes().windows(4);

    let mut in_brackets = false;
    let mut found_abba = false;

    for sec in byte_wins {
        if sec[3] == b'[' || sec[3] == b']' {
            in_brackets = !in_brackets;
            continue;
        }

        // Check for "abba" pattern
        // Middle checked first so that the check will end if a bracket is found
        if sec[2] == sec[1] && sec[0] == sec[3] && sec[0] != sec[1] {
            if in_brackets { return false; }
            found_abba = true;
        }
    }

    found_abba
}

/// Checks if an IPv7 string supports "super-secret listening"
fn supports_listening<S: AsRef<str>>(ipv7: &S) -> bool {
    let byte_wins = ipv7.as_ref().as_bytes().windows(3);

    let mut in_brackets = false;

    // ABA sequences discovered outside brackets, inserted as (A, B)
    let mut critical_pairs_out: Vec<(u8, u8)> = Vec::new();
    // BAB sequences discovered outside brackets, inserted as (A, B)
    let mut critical_pairs_in: Vec<(u8, u8)> = Vec::new();

    for sec in byte_wins {
        if sec[2] == b'[' || sec[2] == b']' {
            in_brackets = !in_brackets;
            continue;
        }

        // Check for "aba" pattern
        if sec[0] == sec[2] && sec[0] != sec[1] {
            let pair: (u8, u8);

            if in_brackets {
                pair = (sec[1], sec[0]);

                if critical_pairs_out.contains(&pair) {
                    return true;
                }

                critical_pairs_in.push(pair)
            } else {
                pair = (sec[0], sec[1]);

                if critical_pairs_in.contains(&pair) {
                    return true;
                }

                critical_pairs_out.push(pair)
            }
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
            110,
            242,
            Pz::new(2016, 7)
        );
    }

    #[test]
    fn ex1() {
        assert!(supports_snooping(&"abba[mnop]qrst"));
        assert!(supports_snooping(&"ioxxoj[asdfgh]zxcvbn"));

        assert!(!supports_snooping(&"abcd[bddb]xyyx"));
        assert!(!supports_snooping(&"aaaa[qwer]tyui"));

        assert!(supports_snooping(&"aaaa[qwegrnerngoer]tuiaaaa[qwer]gnyuiaaaa[qwer]tyyt"));
        assert!(supports_snooping(&"aaaa[qwegrnerngoer]uiiuaaa[qwer]ugnyuiaaaa[qwer]tyui"));
        assert!(!supports_snooping(&"abbangggg[abba]abbageghiehgei"));
    }

    #[test]
    fn ex2() {
        assert!(supports_listening(&"aba[bab]xyz"));
        assert!(supports_listening(&"aaa[kek]eke"));
        assert!(supports_listening(&"zazbz[bzb]cdb"));

        assert!(!supports_listening(&"xyx[xyx]xyx"));
    }
}
