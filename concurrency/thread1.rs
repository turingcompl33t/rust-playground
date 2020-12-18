// thread1.rs

use std::thread;
use std::time::Duration;

fn main() {
    let t = thread::spawn(|| {
        for i in 0..5 {
            println!("Hello from thread {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    t.join().unwrap();
}