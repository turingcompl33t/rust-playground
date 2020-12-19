// puzzle2.rs
//
// Advent of Code Day 9 Puzzle 2.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILENAME: &str = "input.txt";
const TARGET: u64    = 31161678;

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let input: Vec<u64> = reader
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect();

    for window_size in 2..input.len() {
        for w in input.windows(window_size) {
            let a = w.iter().fold(0, |acc, x| acc + x);
            if TARGET == a {
                let min = w.iter().min().unwrap();
                let max = w.iter().max().unwrap();
                println!("{}", min + max);
                return Ok(());
            }
        }
    }
    
    Ok(())
}
