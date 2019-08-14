//! Solution for 2016 Day 23
//!
//! Part two take a good two-three minute to run due to my reluctance
//! to optimize out the addition loops in the assembunny input.

use common::puzzle::{input as pio, PuzzleResult, PuzzleSelection as Pz, Solution};

/// Initial value of register `a` in part one.
const EGG_COUNT_ONE: i32 = 7;

/// Initial value of register `a` in part two.
const EGG_COUNT_TWO: i32 = 12;

pub mod assembunny_toggle;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = parse_instructions(&pio::fetch_lines(puzzle)?)?;

    solve_parts! {
        1 => {
            let mut interpreter = assembunny_toggle::Interpreter::new(
                input.clone()
            );
            interpreter.registers_mut()[&b'a'.into()] = EGG_COUNT_ONE;

            while !interpreter.done() {
                interpreter.execute_next();
            }
            interpreter.registers()[&b'a'.into()]
        },
        2 => {
            let mut interpreter = assembunny_toggle::Interpreter::new(
                input
            );
            interpreter.registers_mut()[&b'a'.into()] = EGG_COUNT_TWO;

            while !interpreter.done() {
                interpreter.execute_next();
            }
            interpreter.registers()[&b'a'.into()]
        }
    }
}

/// Parses the specified lines into assembunny instructions.
///
/// Panics on underlying parse errors.
fn parse_instructions<T>(lines: &[T]) -> Result<Vec<assembunny_toggle::InstrWrapper>, &'static str>
    where T: AsRef<str>
{
    lines.iter().map(|l| l.as_ref().parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    // Take a bit over two minutes to run; ignored by default
    fn solution() {
        assert_solution!(
            11004,
            479_007_564,
            Pz::new(2016, 23)
        )
    }

    #[test]
    fn ex1() {
        use self::assembunny_toggle::Interpreter;

        let instr: [&'static str; 7] = [
            "cpy 2 a",
            "tgl a",
            "tgl a",
            "tgl a",
            "cpy 1 a",
            "dec a",
            "dec a",
        ];

        let mut interp = Interpreter::new(parse_instructions(&instr).unwrap());

        while !interp.done() {
            interp.execute_next();
        }

        assert_eq!(3, interp.registers()[&b'a'.into()]);
    }
}
