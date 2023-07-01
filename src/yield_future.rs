use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use critical_section::Mutex;

pub async fn yield_it() {
    Yield {
        is_yielded: Mutex::new(UnsafeCell::new(false)),
    }
    .await
}

struct Yield {
    is_yielded: Mutex<UnsafeCell<bool>>,
}

impl Future for Yield {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let is_yielded = critical_section::with(|cs| unsafe { *self.is_yielded.borrow(cs).get() });

        if !is_yielded {
            critical_section::with(|cs| unsafe {
                *self.is_yielded.borrow(cs).get() = true;
            });
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
