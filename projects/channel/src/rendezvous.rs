// rendezvous.rs
// Rendezvous channel, like a bounded mpsc channel with 0 internal capacity.

use std::sync::{Arc, Mutex, Condvar};

struct Inner<T> {
    data: Option<T>,
    n_txs: usize,
    n_rxs: usize
}

impl<T> Inner<T> {
    fn new() -> Self {
        Self {
            data: None,
            n_txs: 1,
            n_rxs: 1
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    tx_ok: Condvar,
    rx_ok: Condvar
}

impl<T> Shared<T> {
    fn new() -> Self {
        Self {
            inner: Mutex::new(Inner::<T>::new()),
            tx_ok: Condvar::new(),
            rx_ok: Condvar::new()
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>
}

impl<T> Sender<T> {
    pub fn send(&mut self, data: T) -> Result<(), T> {
        let mut inner = self.shared.inner.lock().unwrap();
        if 0 == inner.n_rxs {
            return Result::Err(data);
        }

        loop {
            match inner.data {
                None => break,
                Some(_) => {
                    inner = self.shared.tx_ok.wait(inner).unwrap();
                }
            }
        }

        inner.data = Some(data);

        drop(inner);
        self.shared.rx_ok.notify_one();

        Ok(())
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.n_txs += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared)
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ()> {
        let mut _inner = self.shared.inner.lock().unwrap();
        Err(())
    }
}

pub fn rendezvous<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::<T>::new());

    (
        Sender {
            shared: Arc::clone(&shared)
        },
        Receiver {
            shared: Arc::clone(&shared)
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (_, _) = rendezvous::<()>();
    }
}