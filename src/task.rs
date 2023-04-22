// Based on https://github.com/rust-embedded-community/async-on-embedded/blob/master/async-embedded/src/executor.rs

use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;

pub struct Task {
    name: &'static str,
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

impl Task {
    pub fn new(name: &'static str, future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            name,
            future: Box::pin(future),
        }
    }

    pub fn future(&mut self) -> Pin<&mut dyn Future<Output = ()>> {
        self.future.as_mut()
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[macro_export]
macro_rules! task {
    ($future:ident) => {
        Task::new(stringify!($future), $future())
    };
}
