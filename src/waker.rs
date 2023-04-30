use crate::executor::EXECUTOR;
use alloc::boxed::Box;
use core::slice::from_raw_parts;
use core::str::from_utf8;
use core::task::{RawWaker, RawWakerVTable, Waker};

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

pub fn waker_id(waker: &Waker) -> &'static str {
    let p = waker.as_raw().data();
    let data = p as *const SizedString as *mut SizedString;
    let sized_string = unsafe { Box::from_raw(data) };
    let sized_string = Box::leak(sized_string);
    let name = unsafe { from_raw_parts(sized_string.string, sized_string.len) };

    let Ok(name) = from_utf8(name) else {
        panic!("name error");
    };

    name
}

pub fn new_waker(name: &'static str) -> Waker {
    unsafe {
        let sized_string: SizedString = name.into();
        let sized_string = Box::new(sized_string);
        let data = Box::into_raw(sized_string) as *const SizedString as *const ();
        Waker::from_raw(RawWaker::new(data, &VTABLE))
    }
}

pub fn delete_waker(waker: &Waker) {
    let p = waker.as_raw().data();
    let data = p as *const SizedString as *mut SizedString;
    let sized_string = unsafe { Box::from_raw(data) };
    drop(sized_string);
}

fn get_name_from_waker(p: *const ()) -> &'static str {
    let data = p as *const SizedString as *mut SizedString;
    let sized_string = unsafe { Box::from_raw(data) };
    let sized_string = Box::leak(sized_string);
    let name = unsafe { from_raw_parts(sized_string.string, sized_string.len) };

    let Ok(name) = from_utf8(name) else {
        panic!("name error");
    };

    name
}

static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        let name = get_name_from_waker(p);

        critical_section::with(|cs| {
            let executor = unsafe { &mut *EXECUTOR.borrow(cs).get() };

            let Some(position) = executor
                .unready_tasks()
                .iter()
                .position(|task| task.name() == name)
                else {
                    panic!("position error");
                };

            let task = executor.unready_tasks().remove(position);
            executor.ready_tasks().push_back(task);
        });
    }
    unsafe fn drop(_: *const ()) {}

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};
