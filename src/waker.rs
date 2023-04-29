use crate::executor::EXECUTOR;
use alloc::sync::Arc;
use core::slice::from_raw_parts;
use core::str::from_utf8;
use core::task::{RawWaker, RawWakerVTable, Waker};

pub fn new_waker(name: &'static str) -> Waker {
    unsafe {
        let sized_string: SizedString = name.into();
        let sized_string = Arc::new(sized_string);
        let data = Arc::into_raw(sized_string);
        let data = data as *const SizedString as *const ();
        Waker::from_raw(RawWaker::new(data, &VTABLE))
    }
}

struct SizedString {
    string: *const u8,
    len: usize,
}

impl From<&'static str> for SizedString {
    fn from(value: &'static str) -> Self {
        Self {
            string: value.as_ptr(),
            len: value.len(),
        }
    }
}

static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        let data = p as *const SizedString as *mut SizedString;
        let sized_string = unsafe { Arc::from_raw(data) };
        let sized_string = sized_string.clone();
        let name = from_raw_parts(sized_string.string, sized_string.len);

        let name = from_utf8(name).unwrap();

        critical_section::with(|cs| {
            let executor = unsafe { &mut *EXECUTOR.borrow(cs).get() };

            let position = executor
                .unready_tasks()
                .iter()
                .position(|task| task.name() == name)
                .unwrap();

            let task = executor.unready_tasks().remove(position);
            executor.ready_tasks().push_back(task);
        });
    }
    unsafe fn drop(_: *const ()) {
        // no-op
    }

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};
