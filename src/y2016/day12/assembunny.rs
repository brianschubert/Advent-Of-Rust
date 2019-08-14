use std::str::FromStr;
use std::ops::{Index, IndexMut};

pub use self::token::*;

mod token {
    use std::str::FromStr;
    use std::num::ParseIntError;

    #[derive(Debug, Copy, Clone)]
    /// A register key from an instruction
    pub struct RegisterKey(u8);

    #[derive(Debug, Copy, Clone)]
    /// A integral literal from an instruction
    pub struct Literal(i16);

    #[derive(Debug, Copy, Clone)]
    /// A value token from an instruction; either a register key or a literal.
    pub enum ValueToken {
        Register(RegisterKey),
        Literal(Literal),
    }

    impl RegisterKey {
        #[inline]
        pub fn key(self) -> u8 { self.0 }
    }

    impl Into<RegisterKey> for u8 {
        fn into(self) -> RegisterKey { RegisterKey(self) }
    }

    impl FromStr for RegisterKey {
        type Err = &'static str;

        fn from_str(token: &str) -> Result<Self, Self::Err> {
            if token.len() != 1 {
                Err("register key must be composed of a single character")
            } else if let key @ b'a' ..= b'd' = *token.as_bytes().first().unwrap() {
                Ok(RegisterKey(key))
            } else { Err("register key must be (a | b | c | d)") }
        }
    }

    impl Literal {
        #[inline]
        pub fn value(self) -> i16 { self.0 }
    }

    impl FromStr for Literal {
        type Err = ParseIntError;

        fn from_str(token: &str) -> Result<Self, Self::Err> {
            Ok(Literal(token.parse()?))
        }
    }


    impl FromStr for ValueToken {
        type Err = &'static str;

        fn from_str(token: &str) -> Result<Self, Self::Err> {
            match token.parse::<Literal>() {
                Ok(lit) => Ok(ValueToken::Literal(lit)),
                Err(_) => token.parse()
                    .map(ValueToken::Register),
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// An assembly-esk assembunny instruction
pub enum Instr {
    Copy(ValueToken, RegisterKey),
    Jnz(ValueToken, ValueToken),
    Inc(RegisterKey),
    Dec(RegisterKey),
}

/// Value held by a register
pub type Register = i32;

#[derive(Debug)]
/// Set of registers used by an assembunny interpreter
pub struct MiniRegisterTable {
    a: Register,
    b: Register,
    c: Register,
    d: Register,
}

#[derive(Debug)]
/// Interpreter for assembunny instructions.
pub struct Interpreter<'a> {
    prog: &'a [Instr],
    pos: usize,
    reg: MiniRegisterTable,
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        let (assem, args) = instr.split_at(3);

        let args: Vec<&str> = args.trim_start().split_whitespace().collect();

        match args.len() {
            1 => match assem {
                "inc" => Ok(Instr::Inc(args[0].parse()?)),
                "dec" => Ok(Instr::Dec(args[0].parse()?)),
                _ => Err("unknown instruction; expected inc/dec from arg count")
            },
            2 => match assem {
                "cpy" => Ok(Instr::Copy(
                    args[0].parse()?,
                    args[1].parse()?,
                )),
                "jnz" => Ok(Instr::Jnz(
                    args[0].parse()?,
                    args[1].parse()
                        .map_err(|_| "could not parse integer")?,
                )),
                _ => Err("unknown instruction; expected cpy/jnz from arg count")
            },
            _ => Err("unknown instruction")
        }
    }
}

impl<'a> Index<&'a RegisterKey> for MiniRegisterTable {
    type Output = i32;

    fn index(&self, index: &'a RegisterKey) -> &Self::Output {
        match index.key() {
            b'a' => &self.a,
            b'b' => &self.b,
            b'c' => &self.c,
            b'd' => &self.d,
            _ => panic!(format!("illegal register index: {:?}", index))
        }
    }
}

impl<'a> IndexMut<&'a RegisterKey> for MiniRegisterTable {
    fn index_mut(&mut self, index: &'a RegisterKey) -> &mut Self::Output {
        match index.key() {
            b'a' => &mut self.a,
            b'b' => &mut self.b,
            b'c' => &mut self.c,
            b'd' => &mut self.d,
            _ => panic!(format!("illegal register index: {:?}", index))
        }
    }
}

impl Default for MiniRegisterTable {
    fn default() -> Self {
        MiniRegisterTable { a: 0, b: 0, c: 0, d: 0 }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(prog: &'a [Instr]) -> Self {
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

        match self.prog[self.pos] {
            Instr::Inc(ref reg) => self.reg[reg] += 1,
            Instr::Dec(ref reg) => self.reg[reg] -= 1,
            Instr::Copy(val, ref reg) => self.reg[reg] = self.token_value(val),
            Instr::Jnz(cond, mag) => if self.token_value(cond) != 0 {
                step = self.token_value(mag) as isize
            }
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
    fn token_value(&self, token: ValueToken) -> Register {
        match token {
            ValueToken::Literal(lit) => i32::from(lit.value()),
            ValueToken::Register(ref key) => self.reg[key],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        assert_eq!(6, "6".parse::<Literal>().unwrap().value());
        assert_eq!(-16, "-16".parse::<Literal>().unwrap().value());

        assert!("a".parse::<Literal>().is_err());
    }

    #[test]
    fn parse_register_key() {
        assert_eq!(b'a', "a".parse::<RegisterKey>().unwrap().key());
        assert_eq!(b'b', "b".parse::<RegisterKey>().unwrap().key());
        assert_eq!(b'c', "c".parse::<RegisterKey>().unwrap().key());
        assert_eq!(b'd', "d".parse::<RegisterKey>().unwrap().key());

        assert!("".parse::<RegisterKey>().is_err());
        assert!("e".parse::<RegisterKey>().is_err());
    }

    #[test]
    fn parse_token() {
        match "a".parse() {
            Ok(ValueToken::Register(ref reg)) => assert_eq!(b'a', reg.key()),
            _ => panic!("failed to parse `a` as value token"),
        }

        match "-7".parse() {
            Ok(ValueToken::Literal(ref lit)) => assert_eq!(-7, lit.value()),
            _ => panic!("failed to parse `a` as value token"),
        }
    }

    #[test]
    fn parse_instruction() {
        match "jnz b -2".parse() {
            Ok(Instr::Jnz(ValueToken::Register(cond), ValueToken::Literal(offset))) => {
                assert_eq!(b'b', cond.key());
                assert_eq!(-2, offset.value());
            }
            _ => panic!("failed to parse jnz instr")
        }

        match "jnz b a".parse() {
            Ok(Instr::Jnz(ValueToken::Register(cond), ValueToken::Register(offset))) => {
                assert_eq!(b'b', cond.key());
                assert_eq!(b'a', offset.key());
            }
            _ => panic!("failed to parse jnz instr")
        }

        match "cpy 26 d".parse() {
            Ok(Instr::Copy(ValueToken::Literal(val), ref reg)) => {
                assert_eq!(26, val.value());
                assert_eq!(b'd', reg.key());
            }
            _ => panic!("failed to parse cpy instr")
        }

        match "inc c".parse() {
            Ok(Instr::Inc(ref reg)) => {
                assert_eq!(b'c', reg.key());
            }
            _ => panic!("failed to parse inc instr")
        }

        match "dec c".parse() {
            Ok(Instr::Dec(ref reg)) => {
                assert_eq!(b'c', reg.key());
            }
            _ => panic!("failed to parse dec instr")
        }
    }
}
