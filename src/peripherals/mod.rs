pub mod gpio;

use core::marker::PhantomData;

#[cfg(feature = "std")]
use crate::std::peripheral::{peripheral_open, peripheral_close, peripheral_write, peripheral_read};
#[cfg(not(feature = "std"))]
use crate::std::peripheral::{peripheral_open, peripheral_close, peripheral_write, peripheral_read};

pub struct Peripheral<T: Default, C: PeripheralKind> {
    config: C,
    _marker: PhantomData<T>,
    id: Option<usize>,
}

pub trait PeripheralKind {
    fn peripheral_kind() -> usize;
}

impl<T: Default, C: PeripheralKind> Peripheral<T, C> {
    pub const fn new(config: C) -> Self {
        Peripheral {
            config,
            _marker: PhantomData,
            id: None,
        }
    }

    pub fn open(&mut self) {
        let id = unsafe {
            let config_ptr = &self.config as *const C as *const ();
            peripheral_open(C::peripheral_kind(), config_ptr)
        };

        self.id = Some(id);
    }

    pub fn write(&mut self, data: T) {
        unsafe {
            let data_ptr = &data as *const T as *const ();
            peripheral_write(C::peripheral_kind(), self.id.unwrap(), data_ptr);
        }
    }

    pub fn read(&self) -> T {
        let mut data = T::default();
        unsafe {
            let data_ptr = &mut data as *mut T as *mut ();
            peripheral_read(C::peripheral_kind(), self.id.unwrap(), data_ptr);
        };
        data
    }

    pub fn get_id(&self) -> usize {
        self.id.unwrap()
    }
}

impl<T: Default, C: PeripheralKind> Drop for Peripheral<T, C> {
    fn drop(&mut self) {
        unsafe {
            peripheral_close(C::peripheral_kind(), self.id.unwrap())
        }
    }
}
