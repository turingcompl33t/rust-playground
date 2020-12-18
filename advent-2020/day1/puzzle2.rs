// puzzle2.rs
//
// Advent of Code Day 1 Puzzle 2.

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use itertools::iproduct;

const FILENAME : &str = "input.txt";

fn main() -> io::Result<()> {
  let f = File::open(FILENAME)?;
  let reader = BufReader::new(f);

  let v : Vec<u64> = reader
      .lines()
      .map(|s| s.unwrap().parse::<u64>().unwrap())
      .collect();

  for (x, y, z) in iproduct!(v.iter(), v.iter(), v.iter()) {
      if x + y + z == 2020 {
          println!("{}", x*y*z);
          break;
      }
  }

  Ok(())
}