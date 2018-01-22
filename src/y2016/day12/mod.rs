//! # Solution for 2016 Day 12

use common::{input as pio, PuzzleSelection as Pz, Solution};

mod assembunny;

pub fn solve(puzzle: Pz) -> Solution {
    let input = parse_instructions(
        &pio::fetch_lines(puzzle).expect("input file could not be parsed")
    );

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
fn parse_instructions<T>(lines: &[T]) -> Vec<assembunny::Instr>
    where T: AsRef<str> + ::std::fmt::Debug
{
    lines.iter().map(|l| l.as_ref().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_solution!(
            318009,
            9227663,
            Pz::of(2016, 12)
        )
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

        let assem_instr = parse_instructions(&instr);

        let mut interp = Interpreter::new(&assem_instr);

        while !interp.done() {
            interp.execute_next();
        }

        assert_eq!(42, interp.registers()[&b'a'.into()]);
    }
}
