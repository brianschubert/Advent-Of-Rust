//! Solution for 2016 Day 05.

use common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_string(puzzle)?;

    solve_parts! {
        both => generate_passwords(input.trim_right().as_bytes())
    }
}

// Both passwords are generated simultaneously to avoid repeating the
// hashing cycle.
fn generate_passwords(door_id: &[u8]) -> (String, String) {
    let mut hasher = Md5::new();

    // Part one's password
    let mut output_one = 0_u32;
    // Whether part one's password has been fully generated
    let mut one_finished = false;

    // Part two's password
    let mut output_two = 0_u32;
    // The positions that have been filled in part two's password.
    // Each bit maps to one nibble (hex char) in the password.
    let mut two_filled = 0_u8;

    for index in 0..u64::max_value() {
        hasher.input(door_id);
        hasher.input(index.to_string().as_bytes());

        let mut result = [0_u8; 16];
        hasher.result(&mut result);

        // Check if first 5 chars in hex representation would be 0
        if result[0] as u16 + result[1] as u16 + (result[2] >> 4) as u16 == 0 {
            if !one_finished {
                output_one = (output_one << 4) + (result[2]) as u32;
                if output_one >> 28 != 0 {
                    one_finished = true;
                }
            }

            // Check if there are empty positions left and that the 6th hex char
            // point to a valid location
            if two_filled != 0xff && result[2] < 8 {
                let pos_offset = (7 - result[2]) * 4;
                // Check that the position hasn't occurred before
                if two_filled & (1 << result[2]) == 0 {
                    // Insert 7th hex char at the designated position
                    output_two += ((result[3] >> 4) as u32) << pos_offset;
                    two_filled |= 1 << result[2];
                    // Drop a print statement here for the "cinematic experience"
                }
            }
        }

        hasher.reset();

        if one_finished && two_filled == 0xff {
            return (format!("{:08x}", output_one), format!("{:08x}", output_two));
        }
    }
    unreachable!()
}

// These tests take several minutes to run so they are ignored by default

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn solution() {
        assert_solution!(
            "801b56a7",
            "424a0197",
            Pz::new(2016, 5)
        )
    }

    #[test]
    #[ignore]
    fn ex_both() {
        let (one, two) = generate_passwords(b"abc");

        assert_eq!("18f47a30", &one[..]);
        assert_eq!("05ace8e3", &two[..]);
    }
}
