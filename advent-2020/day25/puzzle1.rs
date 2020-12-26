// puzzle1.rs
//
// Advent of Code Day 25 Puzzle 1.

use std::fs;

const FILENAME: &str = "input.txt";

fn compute_loop_size(subject: u64, pkey: u64) -> usize {
    let mut val: u64 = 1;
    let mut loop_size = 0;
    loop {
        if val == pkey {
            break;
        }

        val *= subject;
        val = val % 20201227;
        loop_size += 1;
    }
    loop_size
}

fn transform_subject(subject: u64, loop_size: usize) -> u64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject;
        val = val % 20201227;
    }
    val
}

fn main() {
    let vals: Vec<u64> = fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let p1 = vals[0];
    let p2 = vals[1];

    // back out loop size from public key 1
    let loop_size = compute_loop_size(7, p1);

    // compute the encryption key from public key 2 and loop size
    let key = transform_subject(p2, loop_size);

    println!("Encryption key: {}", key);
}
