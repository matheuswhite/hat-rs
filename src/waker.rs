use crate::executor::EXECUTOR;
use alloc::boxed::Box;
use core::task::{RawWaker, RawWakerVTable, Waker};

pub fn waker_id(waker: &Waker) -> u64 {
    let p = waker.as_raw().data();
    let data = p as *const u64 as *mut u64;
    let data = unsafe { Box::from_raw(data) };
    *Box::leak(data)
}

pub fn new_waker(hash: u64) -> Waker {
    unsafe {
        let data = Box::new(hash);
        Waker::from_raw(RawWaker::new(
            Box::into_raw(data) as *const u64 as *const (),
            &VTABLE,
        ))
    }
}

pub fn delete_waker(waker: &Waker) {
    let p = waker.as_raw().data();
    let data = p as *const u64 as *mut u64;
    let data = unsafe { Box::from_raw(data) };
    drop(data);
}

fn get_waker_hash(p: *const ()) -> u64 {
    let data = p as *const u64 as *mut u64;
    let data = unsafe { Box::from_raw(data) };
    *Box::leak(data)
}

static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        let hash = get_waker_hash(p);

        critical_section::with(|cs| {
            let executor = unsafe { &mut *EXECUTOR.borrow(cs).get() };

            executor.set_task_as_ready(hash);
        });
    }
    unsafe fn drop(_: *const ()) {}

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};
