// channels2.rs

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

const N_MSGS : u32 = 5;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = mpsc::Sender::clone(&tx);
    
    let p1 = thread::spawn(move || {
        for i in 0..N_MSGS {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let p2 = thread::spawn(move || {
        for i in 0..N_MSGS {
            tx_clone.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let c = thread::spawn(move || {
        for msg in rx {
            println!("Got: {}", msg);
        }
    });

    p1.join().unwrap();
    p2.join().unwrap();
    c.join().unwrap();
}