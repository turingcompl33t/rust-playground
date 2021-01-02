// unbounded_mpsc.rs
// Muli-producer, single-consumer channel without capacity bound.
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
}

impl<T> Inner<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            n_txs: 1,
            closed: false,
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    tx_post: Condvar,
}

impl<T> Shared<T> {
    fn new(inner: Inner<T>) -> Self {
        Self {
            inner: Mutex::new(inner),
            tx_post: Condvar::new(),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, data: T) -> SendResult<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        match inner.closed {
            true => SendResult::Failure(data),
            false => {
                inner.queue.push_back(data);
                drop(inner);
                self.shared.tx_post.notify_one();
                SendResult::Success
            }
        }
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
            self.shared.tx_post.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(v) = self.buffer.pop_front() {
            return Some(v);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(v) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue)
                    }

                    return Some(v);
                }
                None if 0 == inner.n_txs => return None,
                None => {
                    inner = self.shared.tx_post.wait(inner).unwrap();
                }
            }
        }
    }

    pub fn try_recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.pop_front()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.closed = true;
    }
}

pub struct RecvIterator<T> {
    receiver: Receiver<T>,
}

impl<T> RecvIterator<T> {
    pub fn from_receiver(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }
}

impl<T> Iterator for RecvIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.try_recv()
    }
}

impl<T> IntoIterator for Receiver<T> {
    type Item = T;
    type IntoIter = RecvIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        RecvIterator::from_receiver(self)
    }
}

pub fn unbounded_mpsc<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::<T>::new(Inner::<T>::new()));
    (
        Sender::<T> {
            shared: Arc::clone(&shared),
        },
        Receiver::<T> {
            shared: Arc::clone(&shared),
            buffer: VecDeque::<T>::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let (mut tx, mut rx) = unbounded_mpsc::<i32>();
        assert_eq!(tx.send(1), SendResult::Success);
        assert_eq!(rx.recv().unwrap(), 1);
    }

    #[test]
    fn try_recv_does_not_block() {
        let (_, mut rx) = unbounded_mpsc::<i32>();
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn sender_drops() {
        let (tx, mut rx) = unbounded_mpsc::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn receiver_drops() {
        let (mut tx, rx) = unbounded_mpsc::<i32>();
        drop(rx);
        assert_eq!(tx.send(1), SendResult::Failure(1));
    }

    #[test]
    fn iterator() {
        let (mut tx, rx) = unbounded_mpsc::<i32>();
        assert_eq!(tx.send(1), SendResult::Success);
        assert_eq!(tx.send(2), SendResult::Success);
        let v: Vec<_> = rx.into_iter().collect();
        assert_eq!(v, vec![1, 2]);
    }
}
