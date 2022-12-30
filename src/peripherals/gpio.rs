use core::time::Duration;
use crate::{Expect, Lazy, HATError, Mutex, delay};
use crate::futures::mutex::MutexGuard;
use crate::peripherals::{Peripheral, PeripheralKind};
use heapless::Vec;

#[repr(C, align(4))]
pub struct Gpio {
    port: u32,
    pin: u32,
}

impl PeripheralKind for Gpio {
    fn peripheral_kind() -> usize {
        1
    }
}

const TASK_NUM: usize = 3;
pub const PIN_NUM: usize = 3;
pub const PORT_NUM: usize = 1;

type GpioPeripheral = Peripheral<bool, Gpio>;

type GPIOSTable = Vec<Vec<Mutex<GpioPeripheral, TASK_NUM>, PIN_NUM>, PORT_NUM>;

static GPIOS: Lazy<GPIOSTable> = Lazy::new();

impl Gpio {
    pub fn init() {
        let mut vector: GPIOSTable = Vec::new();

        for _ in 0..PORT_NUM {
            vector.push(Vec::new()).hat_expect("Cannot push port at GPIO Peripheral");
        }

        for (port, pins) in vector.iter_mut().enumerate() {
            for pin in 0..PIN_NUM {
                pins.push(Mutex::new(Peripheral::new(Gpio { port: port as u32, pin: pin as u32 }))).hat_expect("Cannot push pin at GPIO Peripheral");
            }
        }

        GPIOS.init(vector);
    }

    pub async fn new(port: usize, pin: usize) -> Result<MutexGuard<GpioPeripheral, TASK_NUM>, HATError> {
        let gpios = GPIOS.data();

        if port >= gpios.len() {
            return Err(HATError::WrongGPIOPort);
        }

        let pins = gpios.get(port).unwrap();
        if pin >= pins.len() {
            return Err(HATError::WrongGPIOPin);
        }

        let mut gpio = pins.get(pin).unwrap().lock().await;
        gpio.open();

        Ok(gpio)
    }
}

impl Peripheral<bool, Gpio> {
    pub fn toggle(&mut self) {
        if self.read() {
            self.write(false);
        } else {
            self.write(true);
        }
    }

    pub async fn blink(&mut self, period: Duration) {
        self.toggle();
        delay(period).await;
        self.toggle();
        delay(period).await;
    }

    pub async fn n_blink(&mut self, period: Duration, amount: usize) {
        for _ in 0..amount {
            self.blink(period).await;
        }
    }
}
