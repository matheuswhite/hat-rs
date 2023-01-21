use core::future::Future;
use core::pin::Pin;
use core::ptr::NonNull;
use core::task::{Context, Poll};
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::Expect;
use crate::zbus::channel::struct_zbus_channel;
use crate::zbus::no_std::work_queue_backend::WorkQueueState;

pub struct WorkQueueReaderFuture<T: Default> {
    state: ArcMutex<WorkQueueState>,
    channel_reference: NonNull<struct_zbus_channel>,
    data: ArcMutex<Option<T>>,
}

impl<T: Default> WorkQueueReaderFuture<T> {
    pub fn new(channel_reference: NonNull<struct_zbus_channel>) -> Self {
        Self {
            state: Arc::new(BlockingMutex::new(WorkQueueState::None)),
            channel_reference,
            data: Arc::new(BlockingMutex::new(Some(T::default()))),
        }
    }
}

impl<T: Default> Future for WorkQueueReaderFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut data = self.data.lock().hat_expect("Cannot lock data mutex");
        let data_ptr = match data.as_mut() {
            None => panic!("WorkQueueReader's data cannot be 'None'"),
            Some(data) => data,
        };

        let Ok(mut state) = self.state.lock() else {
            panic!("Cannot get work queue future mutex");
        };

        match &mut *state {
            WorkQueueState::None => {
                *state = WorkQueueState::Waiting(cx.waker().clone());

                let state_ptr = Arc::into_raw(self.state.clone()) as *const ();
                unsafe {
                    hat_zbus_read_work_queue(self.channel_reference.as_ptr(),
                                             data_ptr as *const T as *mut T as *mut (),
                                             hat_zbus_work_queue_read_done,
                                             state_ptr);
                }

                Poll::Pending
            }
            WorkQueueState::Waiting(_) => panic!("The \"Waiting\" state must be unreachable at the delay future poll"),
            WorkQueueState::Completed => {
                match data.take() {
                    None => panic!("WorkQueueReader's data cannot be 'None' at 'Completed' state"),
                    Some(data) => Poll::Ready(data),
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn hat_zbus_work_queue_read_done(state: *const ()) {
    let state = unsafe {
        Arc::from_raw(state as *const BlockingMutex<WorkQueueState>)
    };
    let mut state = state.lock()
        .hat_expect("Cannot lock at 'hat_zbus_work_queue_read_done'");

    if let WorkQueueState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    *state = WorkQueueState::Completed;
}

extern "C" {
    fn hat_zbus_read_work_queue(chan: *const struct_zbus_channel, msg: *mut (),
                                callback: unsafe extern "C" fn(*const ()), state: *const ()) -> i32;
}
