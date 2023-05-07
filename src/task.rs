use crate::waker::{delete_waker, new_waker};
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::Waker;

pub struct Task {
    name: &'static str,
    hash: u64,
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

impl Task {
    pub fn new(name: &'static str, hash: u64, future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            name,
            hash,
            future: Box::pin(future),
            waker: new_waker(hash),
        }
    }

    pub fn future_waker(&mut self) -> (Pin<&mut dyn Future<Output = ()>>, &Waker) {
        (self.future.as_mut(), &self.waker)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        delete_waker(&self.waker)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
