use alloc::{collections::VecDeque, sync::Arc};
use core::task::Waker;
use std::sync::Mutex;

/// Channel.
struct Channel<T> {
    queue: VecDeque<T>,
    waker_receiver: Option<Waker>,
    terminated: bool,
}

/// Sender of a channel.
pub struct Sender<T> {
    chan: Arc<Mutex<Channel<T>>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            chan: self.chan.clone(),
        }
    }
}

impl<T> Sender<T> {
    pub async fn send(&self, data: T) -> Result<(), &'static str> {
        let mut chan = self.chan.lock().unwrap();

        if chan.terminated {
            return Err("Connection closed");
        }

        chan.queue.push_back(data);

        Ok(())
    }

    pub fn is_terminated(&self) -> bool {
        let chan = self.chan.lock().unwrap();
        chan.terminated
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut chan = self.chan.lock().unwrap();
        chan.terminated = true;
        if let Some(waker) = chan.waker_receiver.take() {
            waker.wake();
        }
    }
}

pub fn new_sender<T>() -> Sender<T> {
    let chan = Channel {
        queue: Default::default(),
        waker_receiver: None,
        terminated: false,
    };

    let chan = Arc::new(Mutex::new(chan));
    Sender { chan }
}

unsafe impl<T> Send for Sender<T> {}

#[cfg(kani)]
mod tests {
    use super::*;
    use crate::runner;

    #[kani::proof]
    #[kani::unwind(10)]
    fn check() {
        runner::spawnable_block_on(
            async {
                let num: usize = kani::any();
                kani::assume(num <= 5);

                let tx = new_sender();

                let task1 = async move {
                    for i in 0..num {
                        tx.send(i).await.unwrap();
                    }
                };

                let join1 = runner::spawn(task1);

                join1.await;
            },
            runner::RoundRobin::default(),
        );
    }
}
