use crate::hat_panic;
#[cfg(not(feature = "std"))]
use const_format::formatcp;
#[cfg(not(feature = "std"))]
use crate::no_std::panic_fn;
#[cfg(feature = "std")]
use std::sync::PoisonError;
#[cfg(feature = "std")]
use std::sync::mpsc::SendError;
#[cfg(feature = "std")]
use std::sync::mpsc::RecvError;

pub enum HATError {
    Generic,
    MutexPoisonError,
    SendError,
    RecvError,
    LazyNotInit,
    Trigger,
    Semaphore,
    Timer,
    CError(i32),
    WrongGPIOPort,
    WrongGPIOPin,
}

pub trait Expect<T> {
    fn hat_expect(self, msg: &str) -> T;
}

impl<T, E> Expect<T> for Result<T, E> {
    fn hat_expect(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            #[cfg(feature = "std")]
            Err(_e) => hat_panic!("{}", msg),
            #[cfg(not(feature = "std"))]
            Err(_e) => hat_panic!("%s", msg)
        }
    }
}

impl<T> Expect<T> for Option<T> {
    fn hat_expect(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            #[cfg(feature = "std")]
            None => hat_panic!("{}", msg),
            #[cfg(not(feature = "std"))]
            None => hat_panic!("%s", msg)
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<PoisonError<T>> for HATError {
    fn from(_: PoisonError<T>) -> Self {
        HATError::MutexPoisonError
    }
}

#[cfg(feature = "std")]
impl<T> From<SendError<T>> for HATError {
    fn from(_: SendError<T>) -> Self {
        HATError::SendError
    }
}

#[cfg(feature = "std")]
impl From<RecvError> for HATError {
    fn from(_: RecvError) -> Self {
        HATError::RecvError
    }
}

#[cfg(not(feature = "std"))]
impl From<i32> for HATError {
    fn from(_: i32) -> Self {
        HATError::Generic
    }
}
