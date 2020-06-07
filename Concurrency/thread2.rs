// thread2.rs

use std::thread;

fn main() {
    let v = vec![0, 1, 2];

    let t = thread::spawn(move || {
        println!("Got vector: {:?}", v);
    });

    t.join().unwrap();
}