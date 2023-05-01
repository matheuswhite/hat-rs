use crate::waker::waker_id;
use alloc::collections::VecDeque;
use core::cell::UnsafeCell;
use core::future::Future;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

type CSMutex<T> = critical_section::Mutex<T>;

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    is_unlocked: CSMutex<UnsafeCell<bool>>,
    wakers: CSMutex<UnsafeCell<VecDeque<Waker>>>,
}

unsafe impl<T: Sync> Sync for Mutex<T> {}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(T::default())
    }
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            is_unlocked: CSMutex::new(UnsafeCell::new(true)),
            wakers: CSMutex::new(UnsafeCell::new(VecDeque::new())),
        }
    }

    pub async fn lock(&self) -> MutexGuard<T> {
        MutexLocker { mutex: self }.await
    }

    fn unlock(&self) {
        critical_section::with(|cs| {
            let is_unlocked = unsafe { &mut *self.is_unlocked.borrow(cs).get() };
            *is_unlocked = true;
        });
    }
}

pub struct MutexLocker<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexLocker<'a, T> {
    fn contains_waker(wakers: &VecDeque<Waker>, waker: &Waker) -> bool {
        wakers.iter().any(|w| waker_id(w) == waker_id(waker))
    }
}

impl<'a, T> Future for MutexLocker<'a, T> {
    type Output = MutexGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        critical_section::with(|cs| {
            let wakers = unsafe { &mut *self.mutex.wakers.borrow(cs).get() };
            let is_unlocked = critical_section::with(|cs| unsafe {
                &mut *self.mutex.is_unlocked.borrow(cs).get()
            });

            if *is_unlocked {
                if wakers.is_empty() {
                    *is_unlocked = false;
                    Poll::Ready(MutexGuard { mutex: self.mutex })
                } else {
                    let next_waker = wakers.pop_front().unwrap();

                    if waker_id(&next_waker) == waker_id(cx.waker()) {
                        Poll::Ready(MutexGuard { mutex: self.mutex })
                    } else {
                        wakers.push_front(next_waker);

                        if !MutexLocker::<T>::contains_waker(wakers, cx.waker()) {
                            wakers.push_back(cx.waker().clone());
                        }
                        Poll::Pending
                    }
                }
            } else {
                if !MutexLocker::<T>::contains_waker(wakers, cx.waker()) {
                    wakers.push_back(cx.waker().clone());
                }

                Poll::Pending
            }
        })
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn unlock(self) {
        /* Unlocking MutexGuard by dropping */
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();

        critical_section::with(|cs| {
            let wakers = unsafe { &mut *self.mutex.wakers.borrow(cs).get() };

            if let Some(next_waker) = wakers.pop_front() {
                wakers.push_front(next_waker.clone());
                next_waker.wake();
            }
        });
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
