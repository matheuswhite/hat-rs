use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::Waker;
use core::time::Duration;
use critical_section::Mutex;

// TODO Check this
unsafe impl Sync for TimeManager {}

pub struct TimeManager {
    timer: &'static dyn Timer,
    entries: Mutex<UnsafeCell<Vec<(u128, Waker)>>>,
}

impl TimeManager {
    pub const fn new(timer: &'static dyn Timer) -> Self {
        Self {
            timer,
            entries: Mutex::new(UnsafeCell::new(Vec::new())),
        }
    }

    pub fn schedule(&'static self, duration: Duration, waker: Waker) {
        let timeout_instant = self.timer.current_time_ms() + duration.as_millis();

        critical_section::with(|cs| {
            let mut entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries.push((timeout_instant, waker));

            entries.sort_by(|(timeout_a, _), (timeout_b, _)| {
                timeout_a.partial_cmp(timeout_b).unwrap()
            });

            self.timer.start(entries[0].0, self, TimeManager::timeout);
        });
    }

    fn timeout(&'static self) {
        critical_section::with(|cs| {
            let mut entries = unsafe { &mut *self.entries.borrow(cs).get() };

            entries.remove(0).1
        })
        .wake();
    }
}

pub trait Timer {
    fn start(
        &self,
        timeout_ms: u128,
        ctx: &'static TimeManager,
        callback: fn(&'static TimeManager),
    );
    fn current_time_ms(&self) -> u128;
}
