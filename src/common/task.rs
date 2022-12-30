extern crate alloc;

use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use crate::common::UnsafeOption;
use alloc::boxed::Box;
use crate::HATError;

pub type TaskId = usize;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output=T> + 'a>>;
pub type TaskResult = Result<(), HATError>;

pub struct Task {
    id: TaskId,
    future: UnsafeOption<BoxFuture<'static, TaskResult>>,
}

// TODO Explain why this is safe
unsafe impl Sync for Task {}

impl Task {
    pub fn new(id: TaskId, future: impl Future<Output=TaskResult> + 'static) -> Task {
        let box_ = Box::pin(future);

        let task = Task {
            id,
            future: UnsafeCell::new(Some(box_)),
        };

        task
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn future(&self) -> &UnsafeOption<BoxFuture<'static, TaskResult>> {
        &self.future
    }
}
