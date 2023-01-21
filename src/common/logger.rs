#[cfg(not(feature = "std"))]
pub use const_format::formatcp;
#[cfg(not(feature = "std"))]
pub use crate::no_std::{log_fn, trace_fn};
#[cfg(not(feature = "std"))]
pub use crate::no_std::panic_fn;

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! log {
    ($string:literal) => {
        unsafe { log_fn(formatcp!("{}\n\0", $string).as_ptr()) }
    };
    ($format:literal, $($e:expr),+) => {
		unsafe { log_fn(formatcp!("{}\n\0", $format).as_ptr(), $($e),+) }
	};
}

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! safe_log {
    ($string:literal) => {
        log_fn(formatcp!("{}\n\0", $string).as_ptr())
    };
    ($format:literal, $($e:expr),+) => {
		log_fn(formatcp!("{}\n\0", $format).as_ptr(), $($e),+)
	};
}
#[macro_export]
#[cfg(feature = "std")]
macro_rules! log {
    ($string:literal) => {
        println!($string)
    };
    ($format:literal, $($e:expr),+) => {
        println!($format, $($e),+)
    };
}
#[macro_export]
#[cfg(feature = "std")]
macro_rules! safe_log {
    ($string:literal) => {
        println!($string)
    };
}

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! trace {
    () => {
        unsafe { trace_fn(formatcp!("{}\0", file!()).as_ptr(), line!()) }
    };
}

#[macro_export]
#[cfg(feature = "std")]
macro_rules! trace {
    () => {
        println!("{}:{}", file!(), line!());
    };
}

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! hat_panic {
    ($string:literal) => {
        unsafe { panic_fn(formatcp!("{}\n\0", $string).as_ptr()); loop{} }
    };
    ($format:literal, $($e:expr),+) => {
		unsafe { panic_fn(formatcp!("{}\n\0", $format).as_ptr(), $($e),+); loop{} }
	};
}
#[macro_export]
#[cfg(feature = "std")]
macro_rules! mc_panic {
    ($string:literal) => {
        panic!($string)
    };
    ($format:literal, $($e:expr),+) => {
        panic!($format, $($e),+)
    };
}
