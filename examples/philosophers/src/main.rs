#![no_main]
#![no_std]

mod philosophers;

extern crate alloc;

use core::panic::PanicInfo;
use embedded_alloc::Heap;
use hal::prelude::*;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use crate::philosophers::{philosopher, Chopstick};
use hat::prelude::*;

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

    static CHOPSTICKS: [AsyncMutex<Chopstick>; 5] = [
        AsyncMutex::new(Chopstick {}),
        AsyncMutex::new(Chopstick {}),
        AsyncMutex::new(Chopstick {}),
        AsyncMutex::new(Chopstick {}),
        AsyncMutex::new(Chopstick {}),
    ];
    let task_names = [
        "philosopher0",
        "philosopher1",
        "philosopher2",
        "philosopher3",
        "philosopher4",
    ];

    for index in 0..5 {
        let _ = spawn!(task_names[index] => philosopher(index, &CHOPSTICKS));
    }

    rprintln!("End of main task");
}

#[global_allocator]
pub static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 2048;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("panic: {}", info);

    loop {}
}
