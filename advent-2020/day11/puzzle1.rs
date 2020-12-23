// puzzle1.rs
//
// Advent of Code Day 14 Puzzle 1.

use std::{io, str::Chars};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILENAME: &str = "input.txt";

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum CellType {
    Floor,
    Empty,
    Full
}

#[derive(Clone)]
struct Cell {
    celltype: CellType,
}

impl Cell {
    pub fn new(symbol: char) -> Self {
        let t = match symbol {
            '.' => CellType::Floor,
            'L' => CellType::Empty,
            '#' => CellType::Full,
            _   => panic!()
        };

        Self { celltype: t }
    }

    pub fn from_chars(c: Chars) -> Vec<Self> {
        c.into_iter().map(|c| Cell::new(c)).collect()
    }
}

struct SimCtx {
    state1: Vec<Vec<Cell>>,
    state2: Vec<Vec<Cell>>,
    tick: usize
}

impl SimCtx {
    pub fn new(init_state: Vec<Vec<Cell>>) -> Self {
        let c = init_state.clone();
        Self { state1: init_state, state2: c, tick: 0 }
    }
}

fn is_occupied(cell: &Cell) -> bool {
    CellType::Full == cell.celltype
}

fn adjacent_occupied_seats(state: &Vec<Vec<Cell>>, r: usize, c: usize) -> usize {
    let n_rows = state.len();
    let n_cols = state[0].len();

    let left = c.saturating_sub(1);
    let right = c + 1;
    let up = r.saturating_sub(1);
    let down = r + 1;

    let has_left = c > 0;
    let has_right = c < n_cols - 1;
    let has_up = r > 0;
    let has_down = r < n_rows - 1;

    let mut count = 0;
    count += (has_up && has_left && is_occupied(&state[up][left])) as usize;
    count += (has_up && is_occupied(&state[up][c])) as usize;
    count += (has_up && has_right && is_occupied(&state[up][right])) as usize;
    count += (has_left && is_occupied(&state[r][left])) as usize;
    count += (has_right && is_occupied(&state[r][right])) as usize;
    count += (has_down && has_left && is_occupied(&state[down][left])) as usize;
    count += (has_down && is_occupied(&state[down][c])) as usize;
    count += (has_down && has_right && is_occupied(&state[down][right])) as usize;
    count
}

fn simulate_tick(dst: &mut Vec<Vec<Cell>>, src: &Vec<Vec<Cell>>) {
    let n_rows = src.len();
    let n_cols = src[0].len();

    for r in 0..n_rows {
        for c in 0..n_cols {
            // floor cells are never modified
            if CellType::Floor == src[r][c].celltype {
                continue;
            }

            let count = adjacent_occupied_seats(src, r, c);
            if CellType::Empty == src[r][c].celltype && 0 == count {
                dst[r][c].celltype = CellType::Full;
            } else if CellType::Full == src[r][c].celltype && count >= 4 {
                dst[r][c].celltype = CellType::Empty;
            } else {
                dst[r][c].celltype = src[r][c].celltype;
            }
        }
    }
}

fn steady_state(state1: &Vec<Vec<Cell>>, state2: &Vec<Vec<Cell>>) -> bool {
    let n_rows = state1.len();
    let n_cols = state1[0].len();

    for r in 0..n_rows {
        for c in 0..n_cols {
            if state1[r][c].celltype != state2[r][c].celltype {
                return false;
            }
        }
    }
    true
}

fn simulate_to_steady_state(ctx: &mut SimCtx) {
    loop {
        if (ctx.tick & 0x1) > 0 {
            simulate_tick(&mut ctx.state1, &ctx.state2);
        } else {
            simulate_tick(&mut ctx.state2, &ctx.state1);
        }

        if steady_state(&ctx.state1, &ctx.state2) {
            break;
        }

        ctx.tick += 1
    }
}

fn count_occupied_seats(state: &Vec<Vec<Cell>>) -> usize {
    state.iter()
        .flatten()
        .map(|c| (CellType::Full == c.celltype) as usize)
        .sum()
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let init_state: Vec<Vec<Cell>> = reader
        .lines()
        .into_iter()
        .map(|s| Cell::from_chars(s.unwrap().chars()))
        .collect();

    let mut ctx = SimCtx::new(init_state);
    
    simulate_to_steady_state(&mut ctx);
    let count = count_occupied_seats(&ctx.state1);
    println!("{}", count);

    Ok(())
}
