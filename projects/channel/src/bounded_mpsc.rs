// bounded_mpsc.rs
// Multi-producer, single-consumer channel with explicit capacity bound.
//
// Adapted from implementation from Jon Gjengset's stream "Crust of Rust: Channels"

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, PartialEq)]
pub enum SendResult<T> {
    Success,
    Failure(T),
}

struct Inner<T> {
    queue: VecDeque<T>,
    n_txs: usize,
    closed: bool,
    slots: usize,
}

impl<T> Inner<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            n_txs: 1,
            closed: false,
            slots: capacity,
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    tx_ok: Condvar,
    rx_ok: Condvar,
}

impl<T> Shared<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(Inner::<T>::with_capacity(capacity)),
            tx_ok: Condvar::new(),
            rx_ok: Condvar::new(),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, data: T) -> SendResult<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        if inner.closed {
            return SendResult::Failure(data);
        }

        if inner.slots > 0 {
            inner.queue.push_back(data);
            inner.slots -= 1;
            return SendResult::Success;
        }

        // TODO: how to properly use wait_while() here?
        while 0 == inner.slots {
            inner = self.shared.tx_ok.wait(inner).unwrap();
        }

        inner.queue.push_back(data);
        inner.slots -= 1;

        drop(inner);
        self.shared.rx_ok.notify_one();

        SendResult::Success
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.n_txs += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.n_txs -= 1;
        let last_out = 0 == inner.n_txs;
        drop(inner);
        if last_out {
            self.shared.rx_ok.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();

        loop {
            match inner.queue.pop_front() {
                Some(v) => {
                    inner.slots += 1;
                    return Some(v);
                }
                None if 0 == inner.n_txs => return None,
                None => {
                    inner = self.shared.rx_ok.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.closed = true;
        drop(inner);
        self.shared.tx_ok.notify_all();
    }
}

pub fn bounded_mpsc<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    if 0 == capacity {
        panic!("Capacity Must be Positive, Nonzero");
    }

    let shared = Arc::new(Shared::<T>::with_capacity(capacity));
    (
        Sender::<T> {
            shared: Arc::clone(&shared),
        },
        Receiver::<T> {
            shared: Arc::clone(&shared),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (mut tx, mut rx) = bounded_mpsc::<i32>(1);
        assert_eq!(tx.send(1), SendResult::Success);
        assert_eq!(rx.recv().unwrap(), 1);
    }
}
