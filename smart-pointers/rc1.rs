// rc1.rs

use std::rc::Rc;

fn main() {
    let n1 = Rc::new(10);
    println!("Count: {}", Rc::strong_count(&n1));
    let _n2 = Rc::clone(&n1);
    println!("Count: {}", Rc::strong_count(&n1));
    {
        let _n3 = Rc::clone(&n1);
        println!("Count: {}", Rc::strong_count(&n1));
    }
    println!("Count: {}", Rc::strong_count(&n1));
}