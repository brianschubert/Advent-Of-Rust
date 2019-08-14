//! Solution for 2016 Day 02.

use crate::common::puzzle::{input as pio, PuzzleSelection as Pz, Solution, PuzzleResult};
use crate::common::util::Pt;

/// The keypad used to determine the passcode during part one.
const KEYPAD_ONE: [&[char]; 3] = [
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
];

/// The "finger" starting position on the part one keypad.
const START_ONE: Pt<i8> = Pt { x: 1, y: 1 };

/// The keypad used to determine the passcode during part two.
const KEYPAD_TWO: [&[char]; 5] = [
    &['x', 'x', '1', 'x', 'x'],
    &['x', '2', '3', '4', 'x'],
    &['5', '6', '7', '8', '9'],
    &['x', 'A', 'B', 'C', 'x'],
    &['x', 'x', 'D', 'x', 'x'],
];

/// The "finger" starting position on the part two keypad.
const START_TWO: Pt<i8> = Pt { x: 0, y: 2 };

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = pio::fetch_lines(puzzle)?;

    solve_parts! {
        1 => press_keycode(&KEYPAD_ONE, &input, START_ONE)?,
        2 => press_keycode(&KEYPAD_TWO, &input, START_TWO)?
    }
}

/// Returns the passcode resulting from following the specified
/// instructions of the specified keypad.
///
/// An error is returned if an invalid instruction is found.
fn press_keycode<S: AsRef<str>>(
    keypad: &[&[char]],
    instr: &[S],
    start: Pt<i8>,
) -> Result<String, String> {
    // keypad assumed to be a square
    let dim = keypad.len() - 1;
    let mut finger = start; // copy point

    instr.iter().map(|line| {
        for byte in line.as_ref().as_bytes() {
            let next = finger + match *byte {
                b'U' => Pt::<i8>::n(),
                b'R' => Pt::e(),
                b'D' => Pt::s(),
                b'L' => Pt::w(),
                b => return Err(format!("Bad direction: {}", b as char))
            };
            {
                let Pt { x, y } = next;
                if (x >= 0 && x <= dim as i8)
                    && (y >= 0 && y <= dim as i8)
                    && keypad[dim - y as usize][x as usize] != 'x' {
                    finger = next;
                }
            }
        }
        Ok(keypad[dim - finger.y as usize][finger.x as usize])
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            "99332",
            "DD483",
            Pz::new(2016, 2)
        )
    }

    #[test]
    fn ex1() {
        let instr = [
            "ULL",
            "RRDDD",
            "LURDL",
            "UUUUD"
        ];

        assert_eq!(
            "1985",
            press_keycode(&KEYPAD_ONE, &instr, START_TWO).unwrap()
        );
    }

    #[test]
    fn ex2() {
        let instr = [
            "ULL",
            "RRDDD",
            "LURDL",
            "UUUUD"
        ];

        assert_eq!(
            "5DB3",
            press_keycode(&KEYPAD_TWO, &instr, START_TWO).unwrap()
        );
    }
}

