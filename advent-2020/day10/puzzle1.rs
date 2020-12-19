// puzzle1.rs
//
// Advent of Code Day 10 Puzzle 1.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILENAME: &str = "input.txt";

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let mut values: Vec<u32> = reader
        .lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .collect();

    let max: u32 = values.iter().max().unwrap() + 3;
    let min: u32 = 0;

    values.push(max);
    values.push(min);

    values.sort();
    let diffs : Vec<u32> = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(x, y)| y - x)
        .collect();

    let d1 = diffs.iter().filter(|x| **x == 1).count();
    let d2 = diffs.iter().filter(|x| **x == 3).count();
    println!("{}", d1*d2);

    Ok(())
}
