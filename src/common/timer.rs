#[cfg(feature = "std")]
pub use crate::std::timer::timer_init;
#[cfg(feature = "std")]
pub use crate::std::timer::timer_new_delay_ll;

#[cfg(not(feature = "std"))]
pub use crate::no_std::timer::timer_init;
#[cfg(not(feature = "std"))]
pub use crate::no_std::timer::timer_new_delay_ll;


