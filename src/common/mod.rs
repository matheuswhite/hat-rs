pub mod result;
pub mod executor;
pub mod lazy;
pub mod task;
pub mod waker;
pub mod logger;

#[cfg(feature = "std")]
pub mod blocking_channel {
    pub type BlockingChannel<T> = crate::std::blocking_channel::BlockingChannel<T>;
    pub type BlockingSender<T> = crate::std::blocking_channel::BlockingSender<T>;
}

#[cfg(not(feature = "std"))]
pub mod blocking_channel {
    pub type BlockingChannel<T> = crate::no_std::blocking_channel::BlockingChannel<T>;
    pub type BlockingSender<T> = crate::no_std::blocking_channel::BlockingSender<T>;
}


pub mod timer {
    #[cfg(feature = "std")]
    pub use crate::std::timer::timer_init;
    #[cfg(feature = "std")]
    pub use crate::std::timer::timer_new_delay;
    #[cfg(not(feature = "std"))]
    pub use crate::no_std::timer::timer_init;
    #[cfg(not(feature = "std"))]
    pub use crate::no_std::timer::timer_new_delay;
}

#[cfg(feature = "std")]
pub mod blocking_mutex {
    pub type BlockingMutex<T> = std::sync::Mutex<T>;
}

#[cfg(not(feature = "std"))]
pub mod blocking_mutex {
    pub type BlockingMutex<T> = crate::no_std::blocking_mutex::BlockingMutex<T>;
}

#[cfg(feature = "std")]
pub mod arc {
    pub type Arc<T> = std::sync::Arc<T>;
}

#[cfg(not(feature = "std"))]
pub mod arc {
    pub type Arc<T> = crate::no_std::arc::Arc<T>;
}

use core::cell::UnsafeCell;

pub type UnsafeOption<T> = UnsafeCell<Option<T>>;
pub type ArcMutex<T> = crate::common::arc::Arc<crate::common::blocking_mutex::BlockingMutex<T>>;
