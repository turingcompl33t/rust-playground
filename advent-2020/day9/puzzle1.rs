// puzzle1.rs
//
// Advent of Code Day 9 Puzzle 1.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::iproduct;

const FILENAME: &str     = "input.txt";
const WINDOW_SIZE: usize = 26;

fn invalid_window(slice: &[u64], query: u64) -> bool {
    for (x, y) in iproduct!(slice.iter(), slice.iter()) {
        if x == y {
            continue;
        }

        if x + y == query {
            return false;
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let input: Vec<u64> = reader
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect();

    for w in input.windows(WINDOW_SIZE) {
        let (query, slice) = w.split_last().unwrap();
        if invalid_window(slice, *query) {
            println!("First invalid window at: {}", query);
            return Ok(());
        } 
    }

    Ok(())
}
