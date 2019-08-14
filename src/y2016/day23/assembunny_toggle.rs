//! # Assembunny With Toggles
//!
//! Expands upon the assembunny instruction set to include toggled
//! forms of instructions.

use std::str::FromStr;
use y2016::day12::assembunny::{
    Instr,
    ValueToken,
    RegisterKey,
    Literal,
    MiniRegisterTable,
    Register,
};

#[derive(Copy, Clone)]
pub enum InstrWrapper {
    /// An Assembunny Instruction
    Instr(Instr),
    /// A toggle instruction
    Toggle(RegisterKey),
    /// A jnz instruction pretending to be a copy
    MalformedCopy(ValueToken, Literal),
}

impl InstrWrapper {
    /// Converts an assembunny instruction into its toggled form.
    pub fn into_toggled(self) -> InstrWrapper {
        match self {
            InstrWrapper::Instr(instr) => match instr {
                Instr::Dec(reg) => InstrWrapper::Instr(Instr::Inc(reg)),
                Instr::Inc(reg) => InstrWrapper::Instr(Instr::Dec(reg)),
                Instr::Jnz(cond, ValueToken::Register(mag)) =>
                    InstrWrapper::Instr(Instr::Copy(cond, mag)),
                Instr::Jnz(cond, ValueToken::Literal(mag)) =>
                    InstrWrapper::MalformedCopy(cond, mag),
                Instr::Copy(val, reg) => InstrWrapper::Instr(Instr::Jnz(
                    val,
                    ValueToken::Register(reg)
                ))
            }
            InstrWrapper::MalformedCopy(cond, mag) =>
                InstrWrapper::Instr(Instr::Jnz(
                    cond,
                    ValueToken::Literal(mag)
                )),
            InstrWrapper::Toggle(reg) => InstrWrapper::Instr(Instr::Inc(reg)),
        }
    }
}

impl FromStr for InstrWrapper {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(instr) => Ok(InstrWrapper::Instr(instr)),
            Err(e) => match &s[..3] {
                "tgl" => Ok(InstrWrapper::Toggle(s[4..5].parse()?)),
                _ => Err(e)
            }
        }
    }
}

/// Interpreter for toggleable assembunny instructions.
pub struct Interpreter {
    prog: Vec<InstrWrapper>,
    pos: usize,
    reg: MiniRegisterTable,
}

impl Interpreter {
    /// Builds a new interpreter around a set of instructions.
    pub fn new(prog: Vec<InstrWrapper>) -> Self {
        Interpreter {
            prog,
            pos: 0,
            reg: MiniRegisterTable::default(),
        }
    }

    /// Execute the instruction at this interpreters read position and
    /// increments/decrement the position accordingly.
    pub fn execute_next(&mut self) {
        let mut step = 1_isize;
        let instr = self.prog[self.pos];
        match instr {
            InstrWrapper::Instr(ref instr) => {
                match *instr {
                    Instr::Inc(ref reg) => self.reg[reg] += 1,
                    Instr::Dec(ref reg) => self.reg[reg] -= 1,
                    Instr::Copy(ref val, ref reg) => self.reg[reg] = self.token_value(val),
                    Instr::Jnz(ref cond, ref mag) => if self.token_value(cond) != 0 {
                        step = self.token_value(mag) as isize
                    }
                }
            }
            InstrWrapper::Toggle(index) => {
                let instr_pos = self.pos + self.reg[&index] as usize;
                if instr_pos < self.prog.len() {
                    self.prog[instr_pos] = self.prog[instr_pos].into_toggled();
                }
            }
            InstrWrapper::MalformedCopy(..) => () // do nothing
        }

        self.pos = (self.pos as isize + step) as usize;
    }

    /// Returns true if this interpreter has finished executing its
    /// instructions.
    pub fn done(&self) -> bool {
        self.pos >= self.prog.len()
    }

    /// Returns a read-only reference to this interpreters set of registers.
    pub fn registers(&self) -> &MiniRegisterTable {
        &self.reg
    }

    /// Returns a *mutable* reference to this interpreters set of registers.
    pub fn registers_mut(&mut self) -> &mut MiniRegisterTable {
        &mut self.reg
    }

    /// Returns the value associated with the specified value token.
    ///
    /// - If the token is a literal, its value is returned.
    /// - If the token is a register key, the value its respective
    ///   register is returned
    fn token_value(&self, token: &ValueToken) -> Register {
        match *token {
            ValueToken::Literal(lit) => i32::from(lit.value()),
            ValueToken::Register(ref key) => self.reg[key],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instr() {
        // Still parses normal assembuny instructions
        match "jnz b -2".parse() {
            Ok(InstrWrapper::Instr(Instr::Jnz(ValueToken::Register(cond), ValueToken::Literal(offset)))) => {
                assert_eq!(b'b', cond.key());
                assert_eq!(-2, offset.value());
            }
            _ => panic!("failed to parse jnz instr")
        }
        // Also handles the new "toggle" instruction
        match "tgl b".parse() {
            Ok(InstrWrapper::Toggle(reg)) => {
                assert_eq!(b'b', reg.key());
            }
            _ => panic!("failed to parse tgl instr")
        }
    }

    #[test]
    fn toggle_instr() {
        // Inc becomes dec
        match InstrWrapper::Instr("inc a".parse().unwrap()).into_toggled() {
            InstrWrapper::Instr(Instr::Dec(reg)) => {
                assert_eq!(b'a', reg.key());
            }
            _ => panic!("failed to toggle inc instr")
        }
        // Dec becomes inc
        match InstrWrapper::Instr("dec b".parse().unwrap()).into_toggled() {
            InstrWrapper::Instr(Instr::Inc(reg)) => {
                assert_eq!(b'b', reg.key());
            }
            _ => panic!("failed to toggle dec instr")
        }
        // Toggle becomes inc
        match InstrWrapper::Toggle(b'c'.into()).into_toggled() {
            InstrWrapper::Instr(Instr::Inc(reg)) => {
                assert_eq!(b'c', reg.key());
            }
            _ => panic!("failed to toggle toggle instr")
        }
        // jnz(token, reg) becomes copy(token, reg)
        match InstrWrapper::Instr("jnz 17 d".parse().unwrap()).into_toggled() {
            InstrWrapper::Instr(Instr::Copy(ValueToken::Literal(val), reg)) => {
                assert_eq!(17, val.value());
                assert_eq!(b'd', reg.key());
            }
            _ => panic!("failed to toggle jnz(lit, reg) instr")
        }
        // jnz(token, lit) becomes MalformedCopy(token, lit)
        match InstrWrapper::Instr("jnz 17 1".parse().unwrap()).into_toggled() {
            InstrWrapper::MalformedCopy(ValueToken::Literal(val), target) => {
                assert_eq!(17, val.value());
                assert_eq!(1, target.value());
            }
            _ => panic!("failed to toggle jnz(lit, lit) instr")
        }
        // MalformedCopy(token, lit) becomes jnz(token, lit)
        match InstrWrapper::MalformedCopy(
            ValueToken::Literal("17".parse().unwrap()),
            "1".parse().unwrap()
        ).into_toggled() {
            InstrWrapper::Instr(Instr::Jnz(ValueToken::Literal(cond), ValueToken::Literal(offset))) => {
                assert_eq!(17, cond.value());
                assert_eq!(1, offset.value());
            }
            _ => panic!("failed to toggle malformed copy instr")
        }
        // Copy becomes jnz
        match InstrWrapper::Instr("cpy b a".parse().unwrap()).into_toggled() {
            InstrWrapper::Instr(Instr::Jnz(ValueToken::Register(cond), ValueToken::Register(offset))) => {
                assert_eq!(b'b', cond.key());
                assert_eq!(b'a', offset.key());
            }
            _ => panic!("failed to toggle copy instr")
        }
    }
}
