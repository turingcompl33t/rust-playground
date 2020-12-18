// puzzle1.rs
//
// Advent of Code Day 1 Puzzle 1.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use itertools::iproduct;

const FILENAME : &str = "input.txt";

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let v : Vec<u32> = reader
        .lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .collect();

    for (x, y) in iproduct!(v.iter(), v.iter()) {
        if x + y == 2020 {
            println!("{}", x*y);
            break;
        }
    }

    Ok(())
}