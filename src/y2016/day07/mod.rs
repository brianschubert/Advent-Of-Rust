use common::{input as pio, PuzzleSelection as Pz, Solution};

pub fn solve(puzzle: Pz) -> Solution {
    let input = pio::fetch_lines(puzzle)
        .expect("input file could not be read");

    solve_parts! {
        1 => input.iter().filter(|l| supports_snooping(l)).count(),
        2 => input.iter().filter(|l| supports_listening(l)).count()
   }
}

/// Checks if an IPv7 string supports "transport-layer snooping"
fn supports_snooping<S: AsRef<str>>(ipv7: &S) -> bool {
    let bytes = ipv7.as_ref().as_bytes();

    let mut in_brackets = false;
    let mut pos = 0_usize;

    let mut found_abba = false;

    while pos < bytes.len() - 3 {
        let sec: &[u8] = &bytes[pos..pos + 4];

        if sec[3] == b'[' || sec[3] == b']' {
            pos += 4;
            in_brackets = !in_brackets;
            continue;
        }

        // Check for "abba" pattern
        if sec[0] != sec[1] && sec[0] == sec[3] && sec[1] == sec[2] {
            if in_brackets { return false; }
            found_abba = true;
        }
        pos += 1;
    }

    found_abba
}

/// Checks if an IPv7 string supports "super-secret listening"
fn supports_listening<S: AsRef<str>>(ipv7: &S) -> bool {
    let bytes = ipv7.as_ref().as_bytes();

    let mut in_brackets = false;
    let mut pos = 0_usize;

    // ABA sequences discovered outside brackets, inserted as (A, B)
    let mut critical_pairs_out: Vec<(u8, u8)> = Vec::new();
    // BAB sequences discovered outside brackets, inserted as (A, B)
    let mut critical_pairs_in: Vec<(u8, u8)> = Vec::new();

    while pos < bytes.len() - 2 {
        let sec: &[u8] = &bytes[pos..pos + 3];

        if sec[2] == b'[' || sec[2] == b']' {
            pos += 3;
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

        pos += 1;
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
            Pz::of(2016, 7)
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
