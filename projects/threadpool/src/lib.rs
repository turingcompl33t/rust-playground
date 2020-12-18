// lib.rs

use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

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

    pub fn run<F: FnOnce() + Send + 'static>(&self, work: F) {
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
    pub(crate) fn new(id: u8, receiver: Arc::<Mutex::<Receiver::<Job>>>) -> Self {
        let handle = std::thread::spawn(move || {
            println!("[{}] Start", id);
            loop {
                let job_res = receiver.lock().unwrap().recv();
                match job_res {
                    Ok(job) => job(),
                    Err(_) => break
                }
            }
            println!("[{}] Exit", id);
        }); 

        Self {
            id,
            handle
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn basic_pool() {
        let pool = Threadpool::new(2);

        let f = || {
            println!("Message");
            thread::sleep(Duration::from_secs(1));
        };

        pool.run(f.clone());
        pool.run(f);

        // wait for the jobs to complete (bad)
        thread::sleep(Duration::from_secs(3));
    }

    #[test]
    fn concurrent_work() {
        use std::sync::atomic::{AtomicU32, Ordering};

        let pool = Threadpool::new(4);

        let mut count = AtomicU32::new(0);
        let work = || {
            count.fetch_add(1, Ordering::SeqCst);
        };

        pool.run(work);
        pool.run(work);
    }
}
