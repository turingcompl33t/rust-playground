// puzzle1.rs
//
// Advent of Code Day 8 Puzzle 1.

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

fn interpret_until_loop(program: &Vec<Instr>) -> isize {
    let mut observed = HashSet::<isize>::new();
    let mut ip: isize = 0;
    let mut acc: isize = 0;
    loop {
        if observed.contains(&ip) {
            break;
        }

        observed.insert(ip);
        let instr = program.get(ip as usize).expect("IP out of range");
        match instr.opcode {
            Opcode::Acc => { acc += instr.offset; ip += 1; },
            Opcode::Jmp => ip += instr.offset,
            Opcode::Nop => ip += 1,
        }
    }

    acc
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let program: Vec<Instr> = reader
        .lines()
        .map(|s| Instr::new(s.unwrap()))
        .collect();

    let acc = interpret_until_loop(&program);
    println!("Accumulator at first loop: {}", acc);

    Ok(())
}
