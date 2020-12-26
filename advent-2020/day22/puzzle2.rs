// puzzle2.rs
//
// Advent of Code Day 22 Puzzle 2.

use std::{fs, io};
use std::collections::HashSet;
use std::collections::VecDeque;

const FILENAME: &str = "input.txt";

#[derive(PartialEq)]
enum PlayerId {
    One,
    Two,
}

fn encode_round(d: &VecDeque<u64>) -> String {
    d.iter().map(|v| v.to_string()).collect()
}

fn play_recursive_combat(mut d1: VecDeque<u64>, mut d2: VecDeque<u64>) -> (PlayerId, u64) {
    let mut memo: HashSet<String> = HashSet::new();

    while !d1.is_empty() && !d2.is_empty() {
        if !memo.insert(encode_round(&d1)) {
            return (PlayerId::One, 0);
        }

        let t1 = d1.pop_front().unwrap();
        let t2 = d2.pop_front().unwrap();

        let play_subgame = d1.len() as u64 >= t1 && d2.len() as u64 >= t2;
        let (winner, _) = if play_subgame {
            play_recursive_combat(
                d1.iter().copied().take(t1 as usize).collect(),
                d2.iter().copied().take(t2 as usize).collect(),
            )
        } else {
            if t1 > t2 {
                (PlayerId::One, 0)
            } else {
                (PlayerId::Two, 0)
            }
        };

        if PlayerId::One == winner {
            d1.push_back(t1);
            d1.push_back(t2);
        } else {
            d2.push_back(t2);
            d2.push_back(t1);
        }
    }

    let winner = if d1.is_empty() {
        PlayerId::Two
    } else {
        PlayerId::One
    };
    let score: u64 = d1
        .iter()
        .chain(d2.iter())
        .rev()
        .enumerate()
        .map(|(i, &v)| v * ((i + 1) as u64))
        .sum();

    (winner, score)
}

fn main() -> io::Result<()> {
    let mut decks: Vec<VecDeque<u64>> = fs::read_to_string(FILENAME)
        .unwrap()
        .split("\r\n\r\n")
        .into_iter()
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| l.parse::<u64>().unwrap())
                .collect::<VecDeque<u64>>()
        })
        .collect();

    let d2 = decks.pop().unwrap();
    let d1 = decks.pop().unwrap();

    let (_, score) = play_recursive_combat(d1, d2);
    println!("Winning player's score: {}", score);

    Ok(())
}
