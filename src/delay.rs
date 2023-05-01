use crate::time_manager::TimeManager;
use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use critical_section::Mutex;

extern "Rust" {
    static TIME_MANAGER: TimeManager;
}

struct Delay {
    is_ended: Mutex<UnsafeCell<bool>>,
    duration: Duration,
}

pub async fn delay_ms(milli: u64) {
    Delay::new(Duration::from_millis(milli)).await
}

impl Delay {
    pub fn new(duration: Duration) -> Self {
        Self {
            is_ended: Mutex::new(UnsafeCell::new(false)),
            duration,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if critical_section::with(|cs| unsafe { *self.is_ended.borrow(cs).get() }) {
            Poll::Ready(())
        } else {
            critical_section::with(|cs| {
                let is_ended = unsafe { &mut *self.is_ended.borrow(cs).get() };
                *is_ended = true;
            });
            unsafe { TIME_MANAGER.schedule(self.duration, cx.waker().clone()) };
            Poll::Pending
        }
    }
}
