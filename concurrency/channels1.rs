// channels1.rs

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const N_MSGS : u32 = 5;

fn main() {
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for i in 0..N_MSGS {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Got: {}", msg);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}