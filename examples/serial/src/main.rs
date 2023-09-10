#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use embedded_alloc::Heap;
use hat::prelude::*;

/* Change it to your MCU HAL */
use stm32f4xx_hal as hal;
use stm32f4xx_hal::{pac, prelude::*};

use core::fmt::Write;
use embedded_hal::digital::v2::PinState;
use stm32f4xx_hal::gpio::{Output, Pin};
use stm32f4xx_hal::pac::USART3;
use stm32f4xx_hal::serial::{Serial, Tx};

#[hat::main(heap = 1024)]
async fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();

    init_timer(cp.SYST, 16_000_000);

    let gpiod = dp.GPIOD.split();
    let gpiob = dp.GPIOB.split();

    let led1 = gpiob.pb0.into_push_pull_output_in_state(PinState::Low);
    let tx_pin = gpiod.pd8;
    let mut tx = dp.USART3.tx(tx_pin, 115200.bps(), &clocks).unwrap();

    writeln!(tx, "Main task\r\n").unwrap();

    let _ = spawn!("hello" => hello(led1, tx));
}

async fn hello(mut led: Pin<'B', 0, Output>, mut tx: Tx<USART3>) {
    loop {
        writeln!(tx, "Hello, World!\r").unwrap();
        led.toggle();
        delay_ms(100).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
