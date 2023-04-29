#![feature(drain_filter)]
#![feature(waker_getters)]
#![no_main]
#![no_std]

extern crate alloc;

mod delay;
mod executor;
mod task;
mod time_manager;
mod waker;

use crate::delay::delay_ms;
use crate::executor::EXECUTOR;
use crate::task::Task;
use core::panic::PanicInfo;
use hal::prelude::*;
pub use stm32f4xx_hal as hal;

use embedded_alloc::Heap;
use embedded_hal::digital::v2::PinState;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::gpio::{Output, Pin};

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 1024;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("panic: {}", info);

    loop {}
}

#[cortex_m_rt::entry]
fn entry() -> ! {
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    rtt_init_print!();

    rprintln!("Spawning main task...");

    spawn!(main);

    rprintln!("Starting executor...");
    let executor = critical_section::with(|cs| unsafe { &mut *EXECUTOR.borrow(cs).get() });
    executor.block_on().unwrap();

    loop {}
}

async fn main() {
    let dp = hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    rprintln!("Setting up timer...");
    init_timer(cp.SYST);

    rprintln!("Main task init");

    let gpiob = dp.GPIOB.split();
    let led1 = gpiob.pb0.into_push_pull_output_in_state(PinState::Low);
    let led2 = gpiob.pb7.into_push_pull_output_in_state(PinState::Low);

    spawn!("blink" => blink(led1));
    spawn!("blink2" => blink2(led2));

    rprintln!("End of main task");
}

async fn blink(mut led: Pin<'B', 0, Output>) {
    loop {
        delay_ms(500).await;
        led.toggle();
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

        spawn!(say_hello);
    }
}

async fn say_hello() {
    rprintln!("Hello Mr. Matheus");
}

pub use macros::{__current_time_ms, init_timer, TIME_MANAGER};

mod macros {
    use crate::time_manager::TimeManager;
    use crate::HEAP;
    use core::cell::UnsafeCell;
    use cortex_m::peripheral::SYST;
    use cortex_m_rt::exception;
    use critical_section::Mutex;
    use rtt_target::rprintln;

    pub static TIME_MANAGER: TimeManager = TimeManager::new();

    static CTX: &TimeManager = &TIME_MANAGER;
    static CALLBACK: fn(&'static TimeManager, u128) = TimeManager::timeout;
    static TIMER: Mutex<UnsafeCell<Option<SYST>>> = Mutex::new(UnsafeCell::new(None));
    static NOW: Mutex<UnsafeCell<u128>> = Mutex::new(UnsafeCell::new(0));
    static NEXT_TIMEOUT: Mutex<UnsafeCell<u128>> = Mutex::new(UnsafeCell::new(0));

    const SYSTEM_CLOCK: u128 = 8_000_000;

    const TIME_CONTANT: u128 = 10_000_000;
    const TIME_DIVISOR: u128 = 10_000_000_000;
    const TICKS: u128 = (TIME_CONTANT * SYSTEM_CLOCK) / TIME_DIVISOR;

    pub fn init_timer(mut timer: SYST) {
        let ticks = TICKS as u32 & 0x00FF_FFFF;
        rprintln!("Ticks: {}", ticks);

        timer.set_reload(ticks);
        timer.enable_interrupt();
        timer.enable_counter();

        critical_section::with(|cs| {
            let g_timer = unsafe { &mut *TIMER.borrow(cs).get() };

            *g_timer = Some(timer);
        });
    }

    #[no_mangle]
    pub fn __current_time_ms() -> u128 {
        critical_section::with(|cs| *unsafe { &*NOW.borrow(cs).get() })
    }

    #[no_mangle]
    pub fn __start_timer(timeout: u128) {
        critical_section::with(|cs| {
            let next_timeout = unsafe { &mut *NEXT_TIMEOUT.borrow(cs).get() };

            *next_timeout = timeout;
        });
    }

    #[exception]
    fn SysTick() {
        let (next_timeout, now) = critical_section::with(|cs| {
            let timer = unsafe { &mut *TIMER.borrow(cs).get() }.as_mut().unwrap();
            timer.clear_current();

            let now = unsafe { &mut *NOW.borrow(cs).get() };
            *now += 1;

            (unsafe { *NEXT_TIMEOUT.borrow(cs).get() }, *now)
        });

        if next_timeout == now {
            rprintln!("Heap: {}", HEAP.free());
            CALLBACK(CTX, now);
        }
    }
}
