use core::task::Waker;

pub mod backend;
mod reader;
mod publisher;
mod notifier;
mod claimer;
mod waiter;

enum WorkQueueState {
    None,
    Waiting(Waker),
    Completed,
}
