use core::task::Context;
use core::task::Poll;
use crate::common::arc::Arc;
use crate::common::blocking_channel::BlockingChannel;
use crate::common::task::TaskId;
use crate::common::waker::{hat_waker_into_waker, HATWaker};
use crate::common::task::Task;
use heapless::Vec;
use crate::common::result::HATError;
use crate::safe_log;
#[cfg(not(feature = "std"))]
use crate::no_std::log_fn;
#[cfg(not(feature = "std"))]
use const_format::formatcp;

pub struct Executor<const N: usize> {
    tasks: [&'static Task; N],
    active_tasks: BlockingChannel<TaskId>,
}

impl<const N: usize> Executor<N> {
    pub fn new(tasks: Vec<&'static Task, N>) -> Self {
        if let Ok(tasks) = tasks.as_slice().try_into() {
            Executor {
                tasks,
                active_tasks: BlockingChannel::new(),
            }
        } else {
            panic!("Mismatch the number of tasks")
        }
    }

    fn task_by_id(&self, id: TaskId) -> Option<&'static Task> {
        if let Some(task) = self.tasks.iter().find(|&&x| x.id() == id) {
            Some(*task)
        } else {
            None
        }
    }

    pub fn run(&self) -> Result<(), HATError> {
        {
            let guard = self.active_tasks.new_sender();
            let temp_sender = guard.lock()?;
            for task in self.tasks.iter() {
                temp_sender.send(task.id())?;
            }
        }

        let guard = self.active_tasks.new_receiver();
        let receiver = guard.lock()?;
        let mut end_task = 0;

        'main_loop: loop {
            if end_task == self.tasks.len() {
                break 'main_loop;
            }

            let task_id = receiver.recv()?;

            if let Some(task) = self.task_by_id(task_id) {

                // TODO Explain why this is safe
                unsafe {
                    let future_slot = &mut *task.future().get();

                    if let Some(mut future) = future_slot.take() {
                        let hat_waker = Arc::new(
                            HATWaker::new(
                                task.id(),
                                self.active_tasks.new_sender(),
                            )
                        );
                        let waker = hat_waker_into_waker(Arc::into_raw(hat_waker));
                        let context = &mut Context::from_waker(&waker);

                        match future.as_mut().poll(context) {
                            Poll::Pending => *future_slot = Some(future),
                            Poll::Ready(res) => {
                                end_task += 1;
                                match res {
                                    Ok(_) => safe_log!("Task end with success"),
                                    Err(_) => safe_log!("Task end with error"),
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
