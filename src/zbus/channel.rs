use core::marker::PhantomData;
use core::ptr::NonNull;
use crate::zbus::Subscriber;

#[repr(C)]
pub struct struct_zbus_channel {
    _private: [u8; 0],
}

pub trait BackendContructor {
    fn new(channel_ref: NonNull<struct_zbus_channel>) -> Self;
}

pub trait Backend<T> {
    async fn read(&self) -> T;
    async fn publish(&self, data: T);
    async fn notify(&self);
    async fn claim(&self);
    async fn wait_msg(&self) -> T;
}

pub struct Channel<T, B: Backend<T> + BackendContructor> {
    backend: B,
    c_ref: NonNull<struct_zbus_channel>,
    marker: PhantomData<T>,
}

impl<T, B: Backend<T> + BackendContructor> Channel<T, B> {
    pub fn load(c_ref: *const struct_zbus_channel) -> Self {
        let c_ref = unsafe { NonNull::new_unchecked(c_ref as *mut struct_zbus_channel) };
        Self {
            c_ref,
            backend: B::new(c_ref),
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        unsafe {
            name_convert(zbus_channel_name(self.c_ref.as_ptr()))
        }
    }

    pub fn new_subscriber(&self) -> Subscriber<T, B> {
        Subscriber::new(self.c_ref.as_ptr())
    }

    pub async fn claim(&self) -> ClaimedChannel<T> {
        self.backend.claim().await;
        ClaimedChannel::<T> {
            c_ref: self.c_ref,
            marker: PhantomData,
        }
    }

    pub async fn read(&self) -> T {
        self.backend.read().await
    }

    pub async fn publish(&self, data: T) {
        self.backend.publish(data).await
    }

    pub async fn notify(&self) {
        self.backend.notify().await
    }
}

pub struct ClaimedChannel<T> {
    c_ref: NonNull<struct_zbus_channel>,
    marker: PhantomData<T>,
}

impl<T> ClaimedChannel<T> {
    pub fn msg_ref(&self) -> Option<&T> {
        unsafe {
            let msg = zbus_get_msg(self.c_ref.as_ptr()) as *const T;
            if msg.is_null() {
                None
            } else {
                Some(&*msg)
            }
        }
    }

    pub fn msg_mut(&mut self) -> Option<&mut T> {
        unsafe {
            let mut msg = zbus_get_msg(self.c_ref.as_ptr()) as *mut T;
            if msg.is_null() {
                None
            } else {
                Some(&mut *msg)
            }
        }
    }

    pub fn user_data_ref<U>(&self) -> Option<&U> {
        unsafe {
            let user_data = zbus_get_user_data(self.c_ref.as_ptr()) as *const U;
            if user_data.is_null() {
                None
            } else {
                Some(&*user_data)
            }
        }
    }

    pub fn user_data_mut<U>(&mut self) -> Option<&mut U> {
        unsafe {
            let mut user_data = zbus_get_user_data(self.c_ref.as_ptr()) as *mut U;
            if user_data.is_null() {
                None
            } else {
                Some(&mut *user_data)
            }
        }
    }
}

impl<T> Drop for ClaimedChannel<T> {
    fn drop(&mut self) {
        unsafe {
            zbus_finish(self.c_ref.as_ptr());
        }
    }
}

extern "C" {
    fn zbus_channel_name(chan: *const struct_zbus_channel) -> *const u8;
    fn zbus_finish(chan: *const struct_zbus_channel) -> i32;
    fn zbus_get_msg(chan: *const struct_zbus_channel) -> *mut ();
    fn zbus_get_user_data(chan: *const struct_zbus_channel) -> *mut ();
}

unsafe fn rs_strlen(c_str: *const u8) -> usize {
    let mut size = 0;
    let mut ptr = c_str as *mut u8;

    while ptr.read() != 0 {
        ptr = ptr.add(1);
        size += 1;
    }

    size
}

unsafe fn name_convert(c_str: *const u8) -> &'static str {
    let name_len = rs_strlen(c_str);
    let name_slice = core::slice::from_raw_parts(c_str, name_len);
    core::str::from_utf8_unchecked(name_slice)
}
