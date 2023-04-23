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
use crate::time_manager::TimeManager;
use alloc::rc::Rc;

use core::borrow::BorrowMut;
use embedded_alloc::Heap;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 1024;

#[cortex_m_rt::entry]
fn entry() -> ! {
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    rtt_init_print!();

    rprintln!("Setting up timer...");

    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).pclk1(8.MHz()).freeze();

    let mut timer = dp.TIM2.counter(&clocks);

    init_timer(timer);

    rprintln!("Spawning main task...");

    spawn!("main" => main(unsafe { Peripherals::steal() }));

    rprintln!("Starting executor...");
    let executor = critical_section::with(|cs| unsafe { &mut *EXECUTOR.borrow(cs).get() });
    executor.block_on().unwrap();

    loop {}
}

async fn main(_peripherals: Peripherals) {
    rprintln!("Main task init");

    spawn!(blink);

    rprintln!("End of main task");
}

async fn blink() {
    loop {
        rprintln!("LED on");
        delay_ms(1000).await;
        rprintln!("LED off");
        delay_ms(1000).await;
    }
}

pub static TIME_MANAGER: TimeManager = TimeManager::new();

use core::cell::UnsafeCell;
use critical_section::Mutex;
pub use stm32f4xx_hal as hal;

use hal::{
    gpio::{self, Output, PushPull},
    pac::{interrupt, Interrupt, Peripherals, TIM2},
    prelude::*,
    timer::{CounterUs, Event},
};
use stm32f4xx_hal::timer::Counter;

const TIMER_FREQ: u32 = 16_000_000;

static CTX: &TimeManager = &TIME_MANAGER;
static CALLBACK: fn(&'static TimeManager) = TimeManager::timeout;
static TIMER: Mutex<UnsafeCell<Option<Counter<TIM2, TIMER_FREQ>>>> =
    Mutex::new(UnsafeCell::new(None));

fn init_timer(timer: Counter<TIM2, TIMER_FREQ>) {
    critical_section::with(|cs| {
        let g_timer = unsafe { &mut *TIMER.borrow(cs).get() };

        *g_timer = Some(timer);
    });
}

pub fn __current_time_ms() -> u32 {
    critical_section::with(|cs| {
        let timer = unsafe { &mut *TIMER.borrow(cs).get() }.as_ref().unwrap();
        timer.now().duration_since_epoch().to_millis()
    })
}

pub fn __start_timer(timeout: u32) {
    critical_section::with(|cs| {
        let timer = unsafe { &mut *TIMER.borrow(cs).get() }.as_mut().unwrap();
        timer.start(timeout.millis()).unwrap();
    });
}

#[interrupt]
fn TIM2() {
    CALLBACK(CTX);
}
