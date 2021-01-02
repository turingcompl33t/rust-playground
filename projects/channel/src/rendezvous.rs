// rendezvous.rs
// Rendezvous channel, like a bounded mpsc channel with 0 internal capacity.

use std::sync::{Arc, Condvar, Mutex};

struct Inner<T> {
    data: Option<T>,
    n_txs: usize,
    n_rxs: usize,
}

impl<T> Inner<T> {
    fn with_refcounts(n_txs: usize, n_rxs: usize) -> Self {
        Self {
            data: None,
            n_txs,
            n_rxs,
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    tx_ok: Condvar,
    rx_ok: Condvar,
}

impl<T> Shared<T> {
    fn new() -> Self {
        Self {
            inner: Mutex::new(Inner::<T>::with_refcounts(1, 1)),
            tx_ok: Condvar::new(),
            rx_ok: Condvar::new(),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, data: T) -> Result<(), T> {
        let mut inner = self.shared.inner.lock().unwrap();
        if 0 == inner.n_rxs {
            return Err(data);
        }

        loop {
            match inner.data {
                None if 0 == inner.n_rxs => {
                    return Err(data);
                }
                None => {
                    inner.data = Some(data);
                    drop(inner);
                    self.shared.rx_ok.notify_one();
                    return Ok(());
                }
                Some(_) => {
                    inner = self.shared.tx_ok.wait(inner).unwrap();
                }
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
        let last_out = 0 == inner.n_rxs;
        drop(inner);
        if last_out {
            self.shared.rx_ok.notify_all();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ()> {
        let mut inner = self.shared.inner.lock().unwrap();
        if 0 == inner.n_txs {
            return Err(());
        }

        loop {
            match inner.data {
                Some(_) => {
                    let t = inner.data.take().unwrap();
                    drop(inner);
                    self.shared.tx_ok.notify_one();
                    return Ok(t);
                }
                None if 0 == inner.n_txs => {
                    return Err(());
                }
                None => {
                    inner = self.shared.rx_ok.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.n_rxs += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.n_rxs -= 1;
        let last_out = 0 == inner.n_rxs;
        drop(inner);
        if last_out {
            self.shared.tx_ok.notify_all();
        }
    }
}

pub fn rendezvous<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::<T>::new());

    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver {
            shared: Arc::clone(&shared),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (_, _) = rendezvous::<()>();
    }

    #[test]
    fn basic() {
        let (mut tx, mut rx) = rendezvous::<()>();
        assert_eq!(tx.send(()), Ok(()));
        assert_eq!(rx.recv(), Ok(()));
    }

    #[test]
    fn tx_drops() {
        let (tx, mut rx) = rendezvous::<()>();
        drop(tx);
        assert_eq!(rx.recv(), Err(()));
    }

    #[test]
    fn rx_drops() {
        let (mut tx, rx) = rendezvous::<()>();
        drop(rx);
        assert_eq!(tx.send(()), Err(()));
    }
}
