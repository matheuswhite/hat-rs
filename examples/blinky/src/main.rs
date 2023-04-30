#![no_main]
#![no_std]

extern crate alloc;

use core::panic::PanicInfo;
use embedded_alloc::Heap;
use embedded_hal::digital::v2::PinState;
use hal::prelude::*;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::gpio::{Output, Pin};

use hat::prelude::*;

#[global_allocator]
pub static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 1024;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("panic: {}", info);

    loop {}
}

#[hat::main]
async fn main() {
    rtt_init_print!();

    let dp = hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let _ = rcc.cfgr.sysclk(16.MHz()).freeze();

    rprintln!("Setting up timer...");
    init_timer(cp.SYST, 16_000_000);

    rprintln!("Main task init");

    let gpiob = dp.GPIOB.split();
    let led1 = gpiob.pb0.into_push_pull_output_in_state(PinState::Low);
    let led2 = gpiob.pb7.into_push_pull_output_in_state(PinState::Low);

    let _ = spawn!("blink" => blink(led1));
    let _ = spawn!("blink2" => blink2(led2));

    rprintln!("End of main task");
}

async fn blink(mut led: Pin<'B', 0, Output>) {
    let mut count = 0;

    loop {
        delay_ms(500).await;
        led.toggle();
        count += 1;
        delay_ms(500).await;
        led.toggle();
    }
}

async fn blink2(mut led: Pin<'B', 7, Output>) {
    loop {
        delay_ms(100).await;
        led.toggle();
        delay_ms(100).await;
        led.toggle();
    }
}
