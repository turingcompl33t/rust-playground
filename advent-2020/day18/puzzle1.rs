// puzzle1.rs
//
// Advent of Code Day 18 Puzzle 1.

use std::{collections::VecDeque, io};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILENAME: &str = "input.txt";

fn to_rpn(expr: String) -> VecDeque<char> {
    // shunting algorithm to transform expression to RPN
    // NOTE: this implementation assumes that operator +
    // and operator * have equivalent precedence

    let mut queue = VecDeque::<char>::new();
    let mut operators = Vec::<char>::new();
    for c in expr.chars().into_iter() {
        if c.is_digit(10) {
            queue.push_back(c);
        } else if '+' == c || '*' == c {
            while !operators.is_empty() && 
                (*operators.last().unwrap() == '+' || *operators.last().unwrap() == '*') {
                queue.push_back(operators.pop().unwrap());
            }
            operators.push(c);
        } else if '(' == c {
            operators.push(c);
        } else if ')' == c {
            while !operators.is_empty() && *operators.last().unwrap() != '(' {
                queue.push_back(operators.pop().unwrap());
            }
            operators.pop();
        }
    }

    while !operators.is_empty() {
        queue.push_back(operators.pop().unwrap());
    }

    queue
}

fn op(operand1: u64, operand2: u64, operator: char) -> u64 {
    match operator {
        '+' => operand1 + operand2,
        '*' => operand1 * operand2,
        _   => panic!()
    }
}

fn eval(expr: String) -> u64 {
    let rpn = to_rpn(expr);
    let mut stack = Vec::<u64>::new();

    for c in rpn {
        if c.is_digit(10) {
            stack.push(c.to_digit(10).unwrap().into());
        } else {
            // operator
            let operand1 = stack.pop().unwrap();
            let operand2 = stack.pop().unwrap();
            stack.push(op(operand1, operand2, c));
        }     
    }

    *stack.last().unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open(FILENAME)?;
    let reader = BufReader::new(f);

    let result: u64 = reader
        .lines()
        .into_iter()
        .map(|s| s
            .unwrap()
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>())
        .map(|s| eval(s))
        .sum();

    println!("{}", result);

    Ok(())
}
