use heapless::Deque;
use core::task::Waker;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::result::HATError;
use crate::Expect;

pub struct Trigger<const TN: usize> {
    wait_wakers: ArcMutex<Deque<Waker, TN>>,
    is_triggered: AtomicUsize,
}

struct Waiter<const TN: usize> {
    trigger: &'static Trigger<TN>,
}

impl<const TN: usize> Trigger<TN> {
    pub fn new() -> Self {
        Self {
            wait_wakers: Arc::new(BlockingMutex::new(Deque::new())),
            is_triggered: AtomicUsize::new(0),
        }
    }

    pub fn trigger(&'static self) -> Result<(), HATError> {
        let mut wait_wakers = self.wait_wakers.lock()?;

        if self.is_triggered.compare_exchange(0, wait_wakers.len(),
                                              Ordering::AcqRel,
                                              Ordering::Relaxed).is_ok() {
            for waker in wait_wakers.iter() {
                waker.wake_by_ref();
            }
            wait_wakers.clear();
        }

        Ok(())
    }

    pub async fn wait(&'static self) {
        Waiter { trigger: &self }.await
    }
}

impl<const TN: usize> Future for Waiter<TN> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.trigger.is_triggered.load(Ordering::Acquire) == 0 {
            let mut wait_wakers = self.trigger.wait_wakers.lock().hat_expect("Cannot lock wait_wakers at trigger poll");
            wait_wakers.push_back(cx.waker().clone()).hat_expect("Cannot push back the wait waker");
            Poll::Pending
        } else {
            self.trigger.is_triggered.fetch_sub(1, Ordering::AcqRel);
            Poll::Ready(())
        }
    }
}
