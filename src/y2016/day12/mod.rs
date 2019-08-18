//! Solution for 2016 Day 12

use crate::common::puzzle::{input as pio, Result as PuzzleResult, Selection as Pz};

pub mod assembunny;

pub fn solve(puzzle: &Pz) -> PuzzleResult {
    let input = parse_instructions(&pio::fetch_lines(puzzle)?)?;

    solve_parts! {
        1 => {
            let mut interpreter = assembunny::Interpreter::new(&input);

            while !interpreter.done() {
                interpreter.execute_next();
            }
            interpreter.registers()[&b'a'.into()]
        },
        2 => {
            let mut interpreter = assembunny::Interpreter::new(&input);
            interpreter.registers_mut()[&b'c'.into()] = 1;

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
fn parse_instructions<T>(lines: &[T]) -> Result<Vec<assembunny::Instr>, &'static str>
    where T: AsRef<str>
{
    lines.iter().map(|l| l.as_ref().parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(318_009, 9_227_663, Pz::new(2016, 12))
    }

    #[test]
    fn ex1() {
        use self::assembunny::Interpreter;

        let instr: [&'static str; 6] = [
            "cpy 41 a",
            "inc a",
            "inc a",
            "dec a",
            "jnz a 2",
            "dec a",
        ];

        let assem_instr = parse_instructions(&instr).unwrap();

        let mut interp = Interpreter::new(&assem_instr);

        while !interp.done() {
            interp.execute_next();
        }

        assert_eq!(42, interp.registers()[&b'a'.into()]);
    }
}
