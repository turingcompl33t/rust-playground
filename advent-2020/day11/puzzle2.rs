// puzzle2.rs
//
// Advent of Code Day 11 Puzzle 2.

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

fn is_unoccupied(cell: &Cell) -> bool {
    CellType::Empty == cell.celltype
}

fn first_along_vector_occupied(state: &Vec<Vec<Cell>>, r: usize, c: usize, dx: isize, dy: isize) -> bool {
    let mut r = r as isize;
    let mut c = c as isize;

    let n_rows = state.len() as isize;
    let n_cols = state[0].len() as isize;

    r += dy;
    c += dx;
    while (r >= 0) && (r <= n_rows - 1) && (c >= 0) && (c <= n_cols - 1) {
        if is_occupied(&state[r as usize][c as usize]) {
            return true;
        } else if is_unoccupied(&state[r as usize][c as usize]) {
            return false;
        }
        r += dy;
        c += dx;
    }

    false
}

fn visible_occupied_seats(state: &Vec<Vec<Cell>>, r: usize, c: usize) -> usize {
    let mut count = 0;
    count += first_along_vector_occupied(&state, r, c, -1, -1) as usize;
    count += first_along_vector_occupied(&state, r, c, 0, -1) as usize;
    count += first_along_vector_occupied(&state, r, c, 1, -1) as usize;
    count += first_along_vector_occupied(&state, r, c, -1, 0) as usize;
    count += first_along_vector_occupied(&state, r, c, 1, 0) as usize;
    count += first_along_vector_occupied(&state, r, c, -1, 1) as usize;
    count += first_along_vector_occupied(&state, r, c, 0, 1) as usize;
    count += first_along_vector_occupied(&state, r, c, 1, 1) as usize;
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

            let count = visible_occupied_seats(src, r, c);
            if CellType::Empty == src[r][c].celltype && 0 == count {
                dst[r][c].celltype = CellType::Full;
            } else if CellType::Full == src[r][c].celltype && count >= 5 {
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
