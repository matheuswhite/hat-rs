pub mod timer;
pub mod blocking_mutex;
pub mod arc;
pub mod blocking_channel;
#[cfg(not(feature = "std"))]
pub mod alloc_rtos;
#[cfg(all(not(feature = "std"), feature = "peripherals"))]
pub mod peripheral;

extern "C" {
    #[cfg(not(feature = "std"))]
    pub fn log_fn(format: *const u8, ...);
    #[cfg(not(feature = "std"))]
    #[allow(dead_code)]
    pub fn trace_fn(file: *const u8, line: u32);
    #[cfg(not(feature = "std"))]
    pub fn timestamp() -> u32;
    #[cfg(not(feature = "std"))]
    pub fn timestamp_millis() -> u32;
    #[cfg(not(feature = "std"))]
    pub fn panic_fn(format: *const u8, ...);
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() -> ()
{
    loop {}
}
