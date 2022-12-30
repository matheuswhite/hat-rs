use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::time::Duration;
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::timer::timer_new_delay;
use crate::Expect;

pub struct DelayFuture {
    state: ArcMutex<DelayState>,
    timeout: Duration,
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut state) = self.state.lock() {
            match &mut *state {
                DelayState::None => {
                    *state = DelayState::Waiting(cx.waker().clone());
                    timer_new_delay(self.state.clone(), self.timeout.as_millis() as u32).hat_expect("Cannot create a new delay");
                    Poll::Pending
                }
                DelayState::Waiting(_) => {
                    panic!("The \"Waiting\" state must be unreachable at the delay future poll")
                }
                DelayState::Completed => {
                    Poll::Ready(())
                }
            }
        } else {
            panic!("Cannot get delay future mutex");
        }
    }
}

pub enum DelayState {
    None,
    Waiting(Waker),
    Completed,
}

pub fn delay(duration: Duration) -> DelayFuture {
    DelayFuture {
        state: Arc::new(BlockingMutex::new(DelayState::None)),
        timeout: duration,
    }
}
