use std::fs::File;
use std::io::{self, Read};

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let input = std::env::args().skip(1).next().expect("Specify an input");
    println!("Input is {}", input);
    let mut f = File::open(input)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let (_left, program) = parse::program(&s).expect("Invalid program");

    let mut state = State::default();
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&state.pc) {
            break;
        }

        visited.insert(state.pc);

        let instr = &program[state.pc];

        instr.apply(&mut state);
    }

    println!("Part 01: {}", state.acc);

    let mut i = 0;
    loop {
        let mut prgrm = Program::new(&program);
        match prgrm.instr_at(i) {
            Some(Instr::Acc(_)) => { },
            Some(Instr::Nop(off)) => {
                prgrm.code[i] = Instr::Jmp(off);
                if ExitResult::Success == prgrm.run_to_completion() {
                    println!("Part 02: {}", prgrm.state.acc);
                    break
                }
            },
            Some(Instr::Jmp(off)) => {
                prgrm.code[i] = Instr::Nop(off);
                if ExitResult::Success == prgrm.run_to_completion() {
                    println!("Part 02: {}", prgrm.state.acc);
                    break
                }
            },
            None => {
                panic!("Could not find a suitable answer")
            }
        }

        i += 1;
    }


    Ok(())
}

pub struct Program {
    state: State,
    code: Vec<Instr>,
}

impl Program {
    fn new(instructions: &[Instr]) -> Self {
        Program{
            state: Default::default(),
            code: instructions.iter().map(|i| (*i).clone()).collect()
        }
    }

    pub fn next_instr(&self) -> Instr {
        self.code[self.state.pc].clone()
    }

    pub fn instr_at(&self, i: usize) -> Option<Instr> {
        self.code.get(i).map(|x| (*x).clone())
    }

    pub fn run_to_completion(&mut self) -> ExitResult {
        let mut visited = HashSet::new();
        loop {
            if visited.contains(&self.state.pc) {
                return ExitResult::InfiniteLoop;
            }

            if self.state.pc >= self.code.len() {
                return ExitResult::Success
            }

            visited.insert(self.state.pc);

            let instr = &self.code[self.state.pc];

            instr.apply(&mut self.state);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExitResult {
    InfiniteLoop,
    Success,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct State {
    pc: usize,
    acc: i64,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl Instr {
    fn apply(&self, state: &mut State) {
        match self {
            Instr::Nop(_offset) => { state.pc += 1; },
            Instr::Jmp(offset) => { state.pc = (state.pc as i64 + *offset) as usize; },
            Instr::Acc(offset) => { state.pc +=1; state.acc += offset; }
        }
    }
}

mod parse {
    use super::Instr;
    use nom::{
        IResult,
        character::complete::{digit1, line_ending},
        bytes::complete::{tag},
        combinator::{map_res, map},
        sequence::{terminated},
        multi::{many1},
        branch::alt,
    };

    fn number(i: &str) -> IResult<&str, i64> {
        let (i, sign) = alt((map(tag("+"), |_s: &str| 1), map(tag("-"), |_s: &str| -1)))(i)?;
        let (i, num) = map_res(digit1, |s: &str| s.parse::<i64>())(i)?;

        Ok((i, sign * num))
    }

    fn nop(i: &str) -> IResult<&str, Instr> {
        let (i, _) = tag("nop")(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, offset) = number(i)?;

        Ok((i, Instr::Nop(offset)))
    }

    fn jmp(i: &str) -> IResult<&str, Instr> {
        let (i, _) = tag("jmp")(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, num) = number(i)?;

        Ok((i, Instr::Jmp(num)))
    }

    fn acc(i: &str) -> IResult<&str, Instr> {
        let (i, _) = tag("acc")(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, num) = number(i)?;

        Ok((i, Instr::Acc(num)))
    }

    fn line(i: &str) -> IResult<&str, Instr> {
        terminated(alt((nop, jmp, acc)), line_ending)(i)
    }

    pub fn program(i: &str) -> IResult<&str, Vec<Instr>> {
        many1(line)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nop_works() {
        let mut state = State::default();
        Instr::Nop(0).apply(&mut state);
        assert_eq!(1, state.pc);
        assert_eq!(0, state.acc);
    }

    #[test]
    fn jmp_works() {
        let mut state = State::default();
        Instr::Jmp(2).apply(&mut state);
        assert_eq!(2, state.pc);
        assert_eq!(0, state.acc);

        Instr::Jmp(-2).apply(&mut state);
        assert_eq!(0, state.pc);
        assert_eq!(0, state.acc);
    }

    #[test]
    fn acc_works() {
        let mut state = State::default();
        Instr::Acc(25).apply(&mut state);
        assert_eq!(1, state.pc);
        assert_eq!(25, state.acc);
    }
}
