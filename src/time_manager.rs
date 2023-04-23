use crate::{__current_time_ms, __start_timer};
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::Waker;
use core::time::Duration;
use critical_section::Mutex;

// TODO Check this
unsafe impl Sync for TimeManager {}

pub struct TimeManager {
    entries: Mutex<UnsafeCell<Vec<(u32, Waker)>>>,
}

impl TimeManager {
    pub const fn new() -> Self {
        Self {
            entries: Mutex::new(UnsafeCell::new(Vec::new())),
        }
    }

    pub fn schedule(&'static self, duration: Duration, waker: Waker) {
        let timeout_instant = __current_time_ms() + duration.as_millis() as u32;

        critical_section::with(|cs| {
            let mut entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries.push((timeout_instant, waker));

            entries.sort_by(|(timeout_a, _), (timeout_b, _)| {
                timeout_a.partial_cmp(timeout_b).unwrap()
            });

            __start_timer(entries[0].0);
        });
    }

    pub fn timeout(&'static self) {
        critical_section::with(|cs| {
            let mut entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries.remove(0).1
        })
        .wake();
    }
}
