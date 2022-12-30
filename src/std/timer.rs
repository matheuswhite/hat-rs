use core::time::Duration;
use std::cmp::Ordering;
use std::time::Instant;
use crate::common::ArcMutex;
use crate::common::lazy::Lazy;
use crate::common::result::HATError;
use crate::Expect;
use crate::futures::time::DelayState;
use crate::std::blocking_channel::{BlockingChannel, BlockingReceiver};

static CHANNEL: Lazy<BlockingChannel<TimerEntry>> = Lazy::new();

pub fn timer_init() {
    CHANNEL.init(BlockingChannel::new());

    let receiver = CHANNEL.data().new_receiver();
    std::thread::spawn(move || {
        timer_thread(receiver).hat_expect("Timer thread error");
    });
}

pub fn timer_new_delay(state: ArcMutex<DelayState>, timeout: u32) -> Result<(), HATError> {
    let timeout = Instant::now() + Duration::from_millis(timeout as u64);
    let entry = TimerEntry::new(state.clone(), timeout);

    let guard = CHANNEL.data().new_sender();
    let sender = guard.lock()?;
    sender.send(entry)?;

    Ok(())
}

fn timer_thread(input_queue: ArcMutex<BlockingReceiver<TimerEntry>>) -> Result<(), HATError> {
    let mut pending_tasks: Vec<TimerEntry> = vec![];
    let input_queue = input_queue.lock()?;

    loop {
        if !pending_tasks.is_empty() {
            let next_pending_task = pending_tasks.get(0).ok_or(HATError::Timer)?;
            if Instant::now() >= next_pending_task.timeout {
                if let Ok(mut state) = next_pending_task.state.lock() {
                    if let DelayState::Waiting(waker) = &*state {
                        waker.wake_by_ref()
                    }
                    *state = DelayState::Completed
                }
                pending_tasks.remove(0);
            }
        }

        match input_queue.try_recv() {
            Ok(new_entry) => {
                pending_tasks.push(new_entry);
                pending_tasks.sort();
            }
            Err(_err) => {}
        }
    }
}

#[derive(Clone)]
struct TimerEntry {
    state: ArcMutex<DelayState>,
    timeout: Instant,
}

impl TimerEntry {
    fn new(state: ArcMutex<DelayState>, timeout: Instant) -> Self {
        Self {
            state,
            timeout,
        }
    }
}

impl Eq for TimerEntry {}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.timeout < other.timeout {
            Ordering::Less
        } else if self.timeout > other.timeout {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.timeout < other.timeout {
            Some(Ordering::Less)
        } else if self.timeout > other.timeout {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialEq<Self> for TimerEntry {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
