// puzzle2.rs
//
// Advent of Code Day 10 Puzzle 2.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

const FILENAME: &str = "input.txt";

fn path_count_from(from: u32, values: &Vec<u32>, memo: &mut HashMap<u32, usize>) -> usize {
    if memo.contains_key(&from) {
        return *memo.get(&from).unwrap();
    }

    // our device is always the maximum value in the set
    let device = values.iter().max().unwrap();
    
    let total = values
        .iter()
        .filter(|x| **x > from && (**x - from) <= 3)
        .map(|x| if x == device { return 1; } else { return path_count_from(*x, values, memo); } )
        .fold(0, |acc, x| acc + x);
    
    memo.insert(from, total);
    return total;
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let mut values: Vec<u32> = reader
        .lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .collect();

    let min: u32 = 0;
    let max: u32 = values.iter().max().unwrap() + 3;

    values.push(min);
    values.push(max);

    let mut memo = HashMap::<u32, usize>::new();
    
    let count = path_count_from(0, &values, &mut memo);
    println!("Valid configurations: {}", count);

    Ok(())
}
