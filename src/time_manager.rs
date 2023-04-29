use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::Waker;
use core::time::Duration;
use critical_section::Mutex;

extern "Rust" {
    pub fn __current_time_ms() -> u128;
    pub fn __start_timer(timeout: u128);
}

// TODO Check this
unsafe impl Sync for TimeManager {}

pub struct TimeManager {
    entries: Mutex<UnsafeCell<Vec<(u128, Waker)>>>,
}

impl TimeManager {
    pub const fn new() -> Self {
        Self {
            entries: Mutex::new(UnsafeCell::new(Vec::new())),
        }
    }

    pub fn schedule(&'static self, duration: Duration, waker: Waker) {
        let timeout_instant = unsafe { __current_time_ms() } + duration.as_millis();

        critical_section::with(|cs| {
            let entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries.push((timeout_instant, waker));

            entries.sort_by(|(timeout_a, _), (timeout_b, _)| {
                timeout_a.partial_cmp(timeout_b).unwrap()
            });

            unsafe {
                __start_timer(entries[0].0);
            }
        });
    }

    pub fn timeout(&'static self, now: u128) {
        let wakers = critical_section::with(|cs| {
            let entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries
                .drain_filter(|(timeout, _)| *timeout == now)
                .collect::<Vec<_>>()
        });

        wakers.iter().for_each(|(_, waker)| waker.wake_by_ref());
    }
}
