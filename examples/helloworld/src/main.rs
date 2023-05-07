#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use embedded_alloc::Heap;
use hat::prelude::*;
use rtt_target::{rprintln, rtt_init_print};

/* Change it to your MCU HAL */
#[allow(unused_imports)]
use stm32f4xx_hal as hal;

#[hat::main(heap = 1024)]
async fn main() {
    rtt_init_print!();

    rprintln!("Hello, World");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("panic: {}", info);

    loop {}
}
