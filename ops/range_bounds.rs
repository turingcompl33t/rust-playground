// range_bounds.rs

use std::ops::{RangeBounds, Bound};

fn take_a_range<R>(range: R) where R: RangeBounds<usize> {
    println!("Accepted a range");
    match range.start_bound() {
        Bound::Unbounded => println!("Unbounded"),
        Bound::Included(i) => println!("Included({})", i),
        Bound::Excluded(i) => println!("Excluded({})", i)
    }
    match range.end_bound() {
        Bound::Unbounded => println!("Unbounded"),
        Bound::Included(i) => println!("Included({})", i),
        Bound::Excluded(i) => println!("Excluded({})", i)
    }
}

fn main() {
    let range = 0..4;
    take_a_range(range);
}