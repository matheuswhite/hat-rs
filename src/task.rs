use crate::waker::{delete_waker, new_waker};
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::Waker;

pub struct Task {
    name: &'static str,
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

impl Task {
    pub fn new(name: &'static str, future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            name,
            future: Box::pin(future),
            waker: new_waker(name),
        }
    }

    pub fn future_waker(&mut self) -> (Pin<&mut dyn Future<Output = ()>>, &Waker) {
        (self.future.as_mut(), &self.waker)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        delete_waker(&self.waker);
    }
}
