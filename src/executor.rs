use crate::task::Task;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use critical_section::Mutex;

unsafe impl Sync for Executor {}

unsafe impl Send for Executor {}

pub struct Executor {
    ready_tasks: VecDeque<Task>,
    unready_tasks: Vec<Task>,
    is_blocked: bool,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            ready_tasks: VecDeque::new(),
            unready_tasks: Vec::new(),
            is_blocked: false,
        }
    }

    pub fn spawn(&mut self, task: Task) -> Result<(), ()> {
        if self.is_blocked {
            return Err(());
        }

        if self.ready_tasks.iter().any(|x| x.name() == task.name()) {
            return Err(());
        }

        self.ready_tasks.push_back(task);

        Ok(())
    }

    pub fn block_on(&mut self) -> Result<(), ()> {
        if self.is_blocked {
            return Err(());
        }

        self.is_blocked = true;

        while let Some(mut task) = self.ready_tasks.pop_front() {
            let waker = unsafe {
                Waker::from_raw(RawWaker::new(task.name() as *const _ as *const (), &VTABLE))
            };
            let mut cx = Context::from_waker(&waker);

            if let Poll::Pending = task.future().poll(&mut cx) {
                self.unready_tasks.push(task);
            }
        }

        self.is_blocked = false;

        Ok(())
    }
}

#[macro_export]
macro_rules! executor {
    () => {
        critical_section::with(|cs| &mut *EXECUTOR.borrow(cs).get())
    };
}

pub static EXECUTOR: Mutex<UnsafeCell<Executor>> = Mutex::new(UnsafeCell::new(Executor::new()));

static VTABLE: RawWakerVTable = {
    unsafe fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        wake_by_ref(p)
    }
    unsafe fn wake_by_ref(p: *const ()) {
        let name = core::ptr::read(p as *const &str);

        critical_section::with(|cs| {
            let executor = unsafe { &mut *EXECUTOR.borrow(cs).get() };

            let position = executor
                .unready_tasks
                .iter()
                .position(|task| task.name() == name)
                .unwrap();
            let task = executor.unready_tasks.remove(position);
            executor.ready_tasks.push_back(task);
        });
    }
    unsafe fn drop(_: *const ()) {
        // no-op
    }

    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};
