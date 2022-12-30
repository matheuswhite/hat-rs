#![cfg_attr(not(feature = "std"), no_std)]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
#![feature(once_cell)]
#![feature(waker_getters)]

extern crate core;

mod futures;
mod common;
#[cfg(feature = "peripherals")]
mod peripherals;
#[cfg(feature = "std")]
mod std;
#[cfg(not(feature = "std"))]
mod no_std;
#[cfg(feature = "zbus")]
mod zbus;

pub use common::task::{Task, TaskResult};
pub use common::executor::Executor;
pub use common::lazy::Lazy;
pub use common::timer::timer_init;

pub use futures::channel::Channel;
pub use futures::time::delay;
pub use futures::mutex::Mutex;
pub use futures::trigger::Trigger;
pub use futures::semaphore::{Semaphore, SemaphoreUnbounded};

#[macro_export]
macro_rules! trigger {
    ($name:ident, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        static $name: Lazy<Trigger<$task_num>> = Lazy::new_with_init(|| {
            Trigger::new()
        });
    };
}

#[macro_export]
macro_rules! trigger_pub {
    ($name:ident, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        pub static $name: Lazy<Trigger<$task_num>> = Lazy::new_with_init(|| {
            Trigger::new()
        });
    };
}

#[macro_export]
macro_rules! mutex {
    ($name:ident, $val_type:ty, $init_val:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        static $name: Lazy<Mutex<$val_type, $task_num>> = Lazy::new_with_init(|| {
            Mutex::new($init_val)
        });
    };
}

#[macro_export]
macro_rules! mutex_pub {
    ($name:ident, $val_type:ty, $init_val:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        pub static $name: Lazy<Mutex<$val_type, $task_num>> = Lazy::new_with_init(|| {
            Mutex::new($init_val)
        });
    };
}

#[macro_export]
macro_rules! semaphore {
    ($name:ident, $init_count:expr, $total_count:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        static $name: Lazy<Semaphore<$total_count, $task_num>> = Lazy::new_with_init(|| {
            Semaphore::new($init_count)
        });
    };
}

#[macro_export]
macro_rules! semaphore_pub {
    ($name:ident, $init_count:expr, $total_count:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        pub static $name: Lazy<Semaphore<$total_count, $task_num>> = Lazy::new_with_init(|| {
            Semaphore::new($init_count)
        });
    };
}

#[macro_export]
macro_rules! channel {
    ($name:ident, $val_type:ty, $num_of_itens:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        static $name: Lazy<Channel<$val_type, $num_of_itens, $task_num>> = Lazy::new_with_init(|| {
            Channel::new()
        });
    };
}

#[macro_export]
macro_rules! channel_pub {
    ($name:ident, $val_type:ty, $num_of_itens:expr, $task_num:expr) => {
        #[allow(non_upper_case_globals)]
        pub static $name: Lazy<Channel<$val_type, $num_of_itens, $task_num>> = Lazy::new_with_init(|| {
            Channel::new()
        });
    };
}

pub use common::result::{Expect, HATError};
#[cfg(not(feature = "std"))]
pub use no_std::{log_fn, timestamp, timestamp_millis};
#[cfg(not(feature = "std"))]
pub use const_format::formatcp;

#[cfg(all(feature = "std", feature = "peripherals"))]
pub use peripherals::{Peripheral, gpio::Gpio};
#[cfg(all(feature = "std", feature = "peripherals"))]
pub use crate::std::peripheral::read_gpio;

#[cfg(feature = "zbus")]
pub use zbus::error::ZbusError;
#[cfg(feature = "zbus")]
pub use zbus::channel::ZbusChannel;
#[cfg(feature = "zbus")]
pub use zbus::observers::{ZBusListener, ZBusSubscriber};
