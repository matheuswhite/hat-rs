use core::cell::UnsafeCell;
use core::ops::Deref;
use crate::common::UnsafeOption;
use crate::common::result::Expect;

pub type LazyInitFn<T> = fn() -> T;

pub struct Lazy<T: Sized> {
    data: UnsafeOption<T>,
    init: Option<LazyInitFn<T>>,
}

// TODO Explain why this is safe
unsafe impl<T: Sized> Sync for Lazy<T> {}

unsafe impl<T: Sized> Send for Lazy<T> {}

impl<T: Sized> Lazy<T> {
    pub const fn new() -> Self {
        Self {
            data: UnsafeCell::new(None),
            init: None,
        }
    }

    pub const fn new_with_init(init_fn: LazyInitFn<T>) -> Self {
        Self {
            data: UnsafeCell::new(None),
            init: Some(init_fn),
        }
    }

    pub fn init(&self, data: T) {
        // TODO Explain why this is safe
        unsafe {
            let data_ptr = &mut *self.data.get();
            if data_ptr.is_none() {
                *data_ptr = Some(data);
            }
        }
    }

    pub fn data(&self) -> &T {
        // TODO Explain why this is safe
        unsafe {
            let data_ptr = &*self.data.get();
            data_ptr.as_ref().hat_expect("Cannot get the data from empty lazy")
        }
    }
}


impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let data_ptr = &mut *self.data.get();
            if data_ptr.is_none() {
                if let Some(init_fn) = self.init {
                    *data_ptr = Some(init_fn());
                    data_ptr.as_ref().hat_expect("Cannot get the data from empty lazy")
                } else {
                    panic!("Init Fn in None");
                }
            } else {
                data_ptr.as_ref().hat_expect("Cannot get the data from empty lazy")
            }
        }
    }
}
