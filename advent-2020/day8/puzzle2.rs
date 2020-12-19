// puzzle2.rs
//
// Advent of Code Day 8 Puzzle 2.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

const FILENAME: &str = "input.txt";

enum Opcode {
    Acc,
    Nop,
    Jmp
}

struct Instr {
    opcode: Opcode,
    offset: isize,
}

impl Instr {
    pub fn new(line: String) -> Self {
        let s: Vec<&str> = line.split_whitespace().collect();
        let opcode = match s[0] {
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            "nop" => Opcode::Nop,
            _     => unreachable!()
        };
        let offset = s[1].parse::<isize>().unwrap_or(0);
        Self { opcode, offset }
    }
}

fn mutate(instr: &mut Instr) {
    match instr.opcode {
        Opcode::Acc => {},
        Opcode::Jmp => instr.opcode = Opcode::Nop,
        Opcode::Nop => instr.opcode = Opcode::Jmp
    }
}

fn interpret(program: &Vec<Instr>) -> Option<isize> {
    let mut observed = HashSet::<isize>::new();
    let mut ip: isize = 0;
    let mut acc: isize = 0;
    loop {
        if ip as usize == program.len() {
            return Some(acc);
        }

        if observed.contains(&ip) {
            return None;
        }

        observed.insert(ip);
        let instr = program.get(ip as usize).expect("IP out of range");
        match instr.opcode {
            Opcode::Acc => { acc += instr.offset; ip += 1; },
            Opcode::Jmp => ip += instr.offset,
            Opcode::Nop => ip += 1,
        }
    }
}

fn mutate_and_interpret(program: &mut Vec<Instr>) -> isize {
    for i in 0..program.len() {
        mutate(&mut program[i]);
        match interpret(program) {
            None => {},
            Some(acc) => { return acc; }
        }
        mutate(&mut program[i]);
    }

    unreachable!();
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let mut program: Vec<Instr> = reader
        .lines()
        .map(|s| Instr::new(s.unwrap()))
        .collect();

    let acc = mutate_and_interpret(&mut program);
    println!("Accumulator at successful termination: {}", acc);

    Ok(())
}
