use heapless::Deque;
use core::sync::atomic::AtomicUsize;
use core::task::Waker;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::Ordering;
use core::task::{Context, Poll};
use crate::common::arc::Arc;
use crate::common::ArcMutex;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::result::{Expect, HATError};

pub struct SemaphoreUnbounded<const TN: usize> {
    unlocked_count: AtomicUsize,
    wakers: ArcMutex<Deque<Waker, TN>>,
}

struct TakerUnbounded<const TN: usize> {
    semaphore: &'static SemaphoreUnbounded<TN>,
}

impl<const TN: usize> SemaphoreUnbounded<TN> {
    pub fn new(initial_count: usize) -> Self {
        Self {
            unlocked_count: AtomicUsize::new(initial_count),
            wakers: Arc::new(BlockingMutex::new(Deque::new())),
        }
    }

    pub async fn take(&'static self) {
        TakerUnbounded { semaphore: &self }.await
    }

    pub fn give(&self) -> Result<(), HATError> {
        self.unlocked_count.fetch_add(1, Ordering::AcqRel);
        let mut wakers = self.wakers.lock()?;
        if let Some(waker) = wakers.pop_front() {
            waker.wake_by_ref();
        }

        Ok(())
    }
}

impl<const TN: usize> Future for TakerUnbounded<TN> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.semaphore.unlocked_count.load(Ordering::Acquire) > 0 {
            self.semaphore.unlocked_count.fetch_sub(1, Ordering::AcqRel);
            Poll::Ready(())
        } else {
            let mut wakers = self.semaphore.wakers.lock().hat_expect("Cannot lock wakers at take poll");
            wakers.push_back(cx.waker().clone()).hat_expect("Cannot store taker unbounded waker");
            Poll::Pending
        }
    }
}

pub struct Semaphore<const N: usize, const TN: usize> {
    unlocked_count: AtomicUsize,
    max_locks: usize,
    take_wakers: ArcMutex<Deque<Waker, TN>>,
    give_wakers: ArcMutex<Deque<Waker, TN>>,
}

struct Taker<const N: usize, const TN: usize> {
    semaphore: &'static Semaphore<N, TN>,
}

struct Giver<const N: usize, const TN: usize> {
    semaphore: &'static Semaphore<N, TN>,
}

impl<const N: usize, const TN: usize> Semaphore<N, TN> {
    pub fn new(initial_count: usize) -> Self {
        Self {
            unlocked_count: AtomicUsize::new(initial_count),
            max_locks: N,
            take_wakers: Arc::new(BlockingMutex::new(Deque::new())),
            give_wakers: Arc::new(BlockingMutex::new(Deque::new())),
        }
    }

    pub async fn take(&'static self) {
        Taker { semaphore: &self }.await
    }

    pub fn give(&self) -> Result<(), HATError> {
        if self.unlocked_count.load(Ordering::Acquire) < self.max_locks {
            self.unlocked_count.fetch_add(1, Ordering::AcqRel);
            let mut wakers = self.take_wakers.lock()?;
            if let Some(waker) = wakers.pop_front() {
                waker.wake_by_ref();
            }
        }

        Ok(())
    }

    pub async fn wait_give(&'static self) {
        Giver { semaphore: &self }.await
    }
}

impl<const N: usize, const TN: usize> Future for Taker<N, TN> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.semaphore.unlocked_count.load(Ordering::Acquire) > 0 {
            if self.semaphore.unlocked_count.load(Ordering::Acquire) == self.semaphore.max_locks {
                let mut give_wakers = self.semaphore.give_wakers.lock().hat_expect("");
                if let Some(waker) = give_wakers.pop_front() {
                    waker.wake_by_ref();
                }
            }
            self.semaphore.unlocked_count.fetch_sub(1, Ordering::AcqRel);
            Poll::Ready(())
        } else {
            let mut take_wakers = self.semaphore.take_wakers.lock().hat_expect("Cannot lock wakers at take poll");
            take_wakers.push_back(cx.waker().clone()).hat_expect("Cannot store taker waker");
            Poll::Pending
        }
    }
}

impl<const N: usize, const TN: usize> Future for Giver<N, TN> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.semaphore.unlocked_count.load(Ordering::Acquire) < self.semaphore.max_locks {
            if self.semaphore.unlocked_count.load(Ordering::Acquire) == 0 {
                let mut take_wakers = self.semaphore.take_wakers.lock().hat_expect("");
                if let Some(waker) = take_wakers.pop_front() {
                    waker.wake_by_ref();
                }
            }
            self.semaphore.unlocked_count.fetch_add(1, Ordering::AcqRel);
            Poll::Ready(())
        } else {
            let mut give_wakers = self.semaphore.give_wakers.lock().hat_expect("Cannot lock wakers at give poll");
            give_wakers.push_back(cx.waker().clone()).hat_expect("Cannot store giver waker");
            Poll::Pending
        }
    }
}
