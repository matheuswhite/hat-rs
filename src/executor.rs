use crate::task::Task;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::Context;
use critical_section::Mutex;

unsafe impl Sync for Executor {}

unsafe impl Send for Executor {}

pub struct Executor {
    ready_tasks: VecDeque<Task>,
    unready_tasks: Vec<Task>,
    is_blocked: bool,
}

#[derive(Debug)]
pub enum Error {
    TaskNameAlreadyExist,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            ready_tasks: VecDeque::new(),
            unready_tasks: Vec::new(),
            is_blocked: false,
        }
    }

    pub fn spawn(&mut self, task: Task) -> Result<(), Error> {
        if self.ready_tasks.iter().any(|x| x.name() == task.name())
            || self.unready_tasks.iter().any(|x| x.name() == task.name())
        {
            return Err(Error::TaskNameAlreadyExist);
        }

        self.ready_tasks.push_back(task);

        Ok(())
    }

    pub fn block_on(&mut self) -> Result<(), ()> {
        if self.is_blocked {
            return Err(());
        }

        self.is_blocked = true;

        'main_loop: loop {
            while let Some(mut task) = self.ready_tasks.pop_front() {
                let (mut future, waker) = task.future_waker();

                let mut cx = Context::from_waker(waker);

                if future.as_mut().poll(&mut cx).is_pending() {
                    self.unready_tasks.push(task);
                }
            }

            if self.unready_tasks.is_empty() && self.ready_tasks.is_empty() {
                break 'main_loop;
            }
        }

        self.is_blocked = false;

        Ok(())
    }

    pub fn ready_tasks(&mut self) -> &mut VecDeque<Task> {
        &mut self.ready_tasks
    }

    pub fn unready_tasks(&mut self) -> &mut Vec<Task> {
        &mut self.unready_tasks
    }
}

#[macro_export]
macro_rules! spawn {
    ($task:ident) => {
        critical_section::with(|cs| {
            let name = stringify!($task);
            unsafe { &mut *EXECUTOR.borrow(cs).get() }.spawn(Task::new(name, $task()))
        })
    };
    ($task_name:expr => $task_fn:expr) => {
        critical_section::with(|cs| {
            unsafe { &mut *EXECUTOR.borrow(cs).get() }.spawn(Task::new($task_name, $task_fn))
        })
    };
}

pub static EXECUTOR: Mutex<UnsafeCell<Executor>> = Mutex::new(UnsafeCell::new(Executor::new()));
