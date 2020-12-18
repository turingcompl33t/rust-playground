// mutex1.rs

use std::thread;
use std::sync::{Arc, Mutex};

const N_THREADS : u32 = 10;

fn main() {
    let mut handles = vec![]; // Vec::<thread::JoinHandle::<()>>::new();
    
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..N_THREADS {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}