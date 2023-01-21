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

pub struct WorkQueueClaimerFuture {
    state: ArcMutex<WorkQueueState>,
    channel_reference: NonNull<struct_zbus_channel>,
}

impl WorkQueueClaimerFuture {
    pub fn new(channel_reference: NonNull<struct_zbus_channel>) -> Self {
        Self {
            state: Arc::new(BlockingMutex::new(WorkQueueState::None)),
            channel_reference,
        }
    }
}

impl Future for WorkQueueClaimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Ok(mut state) = self.state.lock() else {
            panic!("Cannot get work queue claimer future mutex");
        };

        match &mut *state {
            WorkQueueState::None => {
                *state = WorkQueueState::Waiting(cx.waker().clone());

                let state_ptr = Arc::into_raw(self.state.clone()) as *const ();
                unsafe {
                    hat_zbus_claim_work_queue(self.channel_reference.as_ptr(),
                                              hat_zbus_work_queue_claim_done,
                                              state_ptr);
                }

                Poll::Pending
            }
            WorkQueueState::Waiting(_) => panic!("The \"Waiting\" state must be unreachable at the work queue claimer future poll"),
            WorkQueueState::Completed => Poll::Ready(())
        }
    }
}

#[no_mangle]
pub extern "C" fn hat_zbus_work_queue_claim_done(state: *const ()) {
    let state = unsafe {
        Arc::from_raw(state as *const BlockingMutex<WorkQueueState>)
    };
    let mut state = state.lock()
        .hat_expect("Cannot lock at 'hat_zbus_work_queue_claim_done'");

    if let WorkQueueState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    *state = WorkQueueState::Completed;
}

extern "C" {
    fn hat_zbus_claim_work_queue(chan: *const struct_zbus_channel,
                                 callback: unsafe extern "C" fn(*const ()), state: *const ()) -> i32;
}
