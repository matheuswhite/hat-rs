use crate::futures::time::DelayState;
use crate::no_std::arc::Arc;
use crate::common::blocking_mutex::BlockingMutex;
use crate::common::result::Expect;
use crate::HATError;

pub fn timer_init() {
    // TODO Explain why this is safe
    unsafe {
        rtos_timer_init();
    }
}

pub fn timer_new_delay(state: Arc<BlockingMutex<DelayState>>, timeout: u32) -> Result<(), HATError> {
    let state_ptr = Arc::into_raw(state) as *const ();

    // TODO Explain why this is safe
    unsafe {
        rtos_timer_reschedule(rtos_timer_timeout, state_ptr, timeout);
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn rtos_timer_timeout(state: *const ()) {
    // TODO Explain why this is safe
    let state = unsafe { Arc::from_raw(state as *const BlockingMutex<DelayState>) };
    let mut state = state.lock().hat_expect("Cannot lock at rtos timer timeout");

    if let DelayState::Waiting(waker) = &*state {
        waker.wake_by_ref();
    }
    *state = DelayState::Completed;
}

extern "C" {
    pub fn rtos_timer_init();
    pub fn rtos_timer_reschedule(callback: unsafe extern "C" fn(*const ()), state: *const (), timeout: u32);
}
