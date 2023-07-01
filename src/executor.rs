use crate::task::Task;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::task::Context;
use critical_section::Mutex;
use rtt_target::rprintln;

unsafe impl Sync for Executor {}

unsafe impl Send for Executor {}

pub struct Executor {
    ready_tasks: VecDeque<Task>,
    unready_tasks: Vec<Task>,
    is_blocked: bool,
    current_task: Option<Task>,
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
            current_task: None,
        }
    }

    pub fn spawn(&mut self, task: Task) -> Result<(), Error> {
        if self.ready_tasks.iter().any(|x| x == &task)
            || self.unready_tasks.iter().any(|x| x == &task)
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
            while !self.ready_tasks.is_empty() {
                self.current_task = self.ready_tasks.pop_front();

                let task = self.current_task.as_mut().unwrap();

                let (mut future, waker) = task.future_waker();

                let mut cx = Context::from_waker(waker);

                if future.as_mut().poll(&mut cx).is_pending() {
                    if let Some(task) = self.current_task.take() {
                        self.unready_tasks.push(task);
                    }

                    self.print_tasks();
                }
            }

            if self.unready_tasks.is_empty() && self.ready_tasks.is_empty() {
                break 'main_loop;
            }
        }

        self.is_blocked = false;

        Ok(())
    }

    fn print_tasks(&self) {
        self.ready_tasks
            .iter()
            .for_each(|task| rprintln!("R Task [{:#x}]: {}", task.hash(), task.name()));
        self.unready_tasks
            .iter()
            .for_each(|task| rprintln!("UR Task [{:#x}]: {}", task.hash(), task.name()));
    }

    pub fn ready_tasks(&mut self) -> &mut VecDeque<Task> {
        &mut self.ready_tasks
    }

    pub fn unready_tasks(&mut self) -> &mut Vec<Task> {
        &mut self.unready_tasks
    }

    pub fn set_task_as_ready(&mut self, hash: u64) {
        if let Some(task) = self.current_task.take() {
            if task.hash() == hash {
                self.ready_tasks.push_back(task);
                return;
            }

            self.current_task = Some(task);
        }

        let Some(position) = self.unready_tasks.iter().position(|task| task.hash() == hash) else {
            panic!("Task with hash {:#x} not exist!", hash);
        };

        let task = self.unready_tasks.remove(position);
        self.ready_tasks.push_back(task);
    }
}

#[macro_export]
macro_rules! hash {
    ($expr:expr) => {{
        let mut hasher = FxHasher::default();
        ($expr).hash(&mut hasher);
        hasher.finish()
    }};
}

#[macro_export]
macro_rules! spawn {
    ($task:ident) => {
        critical_section::with(|cs| {
            let name = stringify!($task);
            unsafe { &mut *EXECUTOR.borrow(cs).get() }.spawn(Task::new(name, hash!(name), $task()))
        })
    };
    ($task_name:expr => $task_fn:expr) => {
        critical_section::with(|cs| {
            unsafe { &mut *EXECUTOR.borrow(cs).get() }.spawn(Task::new(
                $task_name,
                hash!($task_name),
                $task_fn,
            ))
        })
    };
}

pub static EXECUTOR: Mutex<UnsafeCell<Executor>> = Mutex::new(UnsafeCell::new(Executor::new()));
