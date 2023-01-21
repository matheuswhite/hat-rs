use core::future::Future;
use core::pin::Pin;
use core::ptr::NonNull;
use core::task::{Context, Poll};
use crate::common::ArcMutex;
use crate::Expect;
use crate::no_std::arc::Arc;
use crate::no_std::blocking_mutex::BlockingMutex;
use crate::zbus::channel::struct_zbus_channel;
use crate::zbus::no_std::work_queue_backend::WorkQueueState;

pub struct WorkQueuePublisherFuture<T: Default> {
    state: ArcMutex<WorkQueueState>,
    channel_reference: NonNull<struct_zbus_channel>,
    data: T,
}

impl<T: Default> WorkQueuePublisherFuture<T> {
    pub fn new(channel_reference: NonNull<struct_zbus_channel>, data: T) -> Self {
        Self {
            state: Arc::new(BlockingMutex::new(WorkQueueState::None)),
            channel_reference,
            data,
        }
    }
}

impl<T: Default> Future for WorkQueuePublisherFuture<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Ok(mut state) = self.state.lock() else {
            panic!("Cannot get work queue publisher future mutex");
        };

        match &mut *state {
            WorkQueueState::None => {
                *state = WorkQueueState::Waiting(cx.waker().clone());

                let state_ptr = Arc::into_raw(self.state.clone()) as *const ();
                unsafe {
                    hat_zbus_publish_work_queue(self.channel_reference.as_ptr(),
                                                &self.data as *const T as *const (),
                                                hat_zbus_work_queue_publish_done,
                                                state_ptr);
                }

                Poll::Pending
            }
            WorkQueueState::Waiting(_) => panic!("The \"Waiting\" state must be unreachable at the work queue publisher future poll"),
            WorkQueueState::Completed => Poll::Ready(()),
        }
    }
}

#[no_mangle]
pub extern "C" fn hat_zbus_work_queue_publish_done(state: *const ()) {
    let state = unsafe {
        Arc::from_raw(state as *const BlockingMutex<WorkQueueState>)
    };
    let mut state = state.lock()
        .hat_expect("Cannot lock at 'hat_zbus_work_queue_publish_done'");

    if let WorkQueueState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    *state = WorkQueueState::Completed
}

extern "C" {
    fn hat_zbus_publish_work_queue(chan: *const struct_zbus_channel, msg: *const (),
                                   callback: unsafe extern "C" fn(*const ()), state: *const ()) -> i32;
}
