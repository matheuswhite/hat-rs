use core::marker::PhantomData;
use core::ptr::NonNull;
use crate::zbus::struct_zbus_channel;
use crate::zbus::channel::{Backend, BackendContructor};

pub struct Subscriber<T, B: Backend<T> + BackendContructor> {
    backend: B,
    c_ref: NonNull<struct_zbus_channel>,
    marker: PhantomData<T>,
}

impl<T, B: Backend<T> + BackendContructor> Subscriber<T, B> {
    pub fn new(c_ref: *const struct_zbus_channel) -> Self {
        let c_ref = unsafe { NonNull::new_unchecked(c_ref as *mut struct_zbus_channel) };
        let backend = B::new(c_ref);

        Self {
            c_ref,
            backend,
            marker: PhantomData,
        }
    }

    pub async fn wait_msg(&self) -> T {
        self.backend.wait_msg().await
    }
}
