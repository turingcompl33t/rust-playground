// lib.rs

use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

struct Threadpool {
    workers: Vec::<Worker>,
    sender: Sender::<Job>
}

impl Threadpool {
    pub fn new(count: u8) -> Self {
        let (tx, rx) = channel::<Job>();

        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::<Worker>::new();
        for id in 0..count {
            let c = Arc::clone(&receiver);
            workers.push(Worker::new(id, c));
        }

        Self {
            workers,
            sender: tx
        }
    }

    pub fn run<F: FnOnce() + 'static>(&self, work: F) {
        self.sender
            .send(Box::new(work))
            .unwrap();
    }
}

struct Worker {
    id: u8,
    handle: JoinHandle<()>
}

impl Worker {
    pub fn new(id: u8, receiver: Arc::<Mutex::<Receiver::<Job>>>) -> Self {
        let handle = std::thread::spawn(move || {
            // TODO
        }); 
        Self {
            id,
            handle
        }
    }
}

type Job = Box<dyn FnOnce() + 'static>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_pool() {
        let pool = Threadpool::new(4);
        pool.run(|| println!("hello!"));
        
        assert_eq!(2 + 2, 4);
    }
}
