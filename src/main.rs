#![no_main]
#![no_std]

extern crate alloc;

mod delay;
mod executor;
mod task;
mod time_manager;

use crate::delay::delay_ms;
use crate::executor::EXECUTOR;
use crate::task::Task;

use crate::time_manager::{TimeManager, Timer};
use embedded_alloc::Heap;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 1024;

#[cortex_m_rt::entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    rtt_init_print!();

    rprintln!("Hello, world!");

    unsafe {
        let executor = executor!();

        executor.spawn(task!(blink)).unwrap();

        executor.block_on().unwrap();
    }

    loop {}
}

async fn blink() {
    loop {
        rprintln!("LED on");
        delay_ms(1000).await;
        rprintln!("LED off");
        delay_ms(1000).await;
    }
}

pub static TIME_MANAGER: TimeManager = TimeManager::new(&TIMER);

static TIMER: STM32Timer = STM32Timer {};

struct STM32Timer {}

impl Timer for STM32Timer {
    fn start(
        &self,
        timeout_ms: u128,
        ctx: &'static TimeManager,
        callback: fn(&'static TimeManager),
    ) {
        todo!()
    }

    fn current_time_ms(&self) -> u128 {
        todo!()
    }
}
