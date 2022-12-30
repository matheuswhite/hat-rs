use core::marker::PhantomData;
use core::ptr::NonNull;
use crate::common::ArcMutex;
use crate::common::result::Expect;

pub struct BlockingChannel<T: Default> {
    ptr: NonNull<()>,
    maker: PhantomData<T>,
}

pub struct BlockingSender<T> {
    ptr: NonNull<()>,
    maker: PhantomData<T>,
}

pub struct BlockingReceiver<T: Default> {
    ptr: NonNull<()>,
    maker: PhantomData<T>,
}

impl<T: Default> BlockingChannel<T> {
    pub fn new() -> Self {
        BlockingChannel {
            ptr: NonNull::new(unsafe { rtos_msgq_new(core::mem::size_of::<T>()) }).hat_expect("The pointer is non-null"),
            maker: PhantomData,
        }
    }

    pub fn new_sender(&self) -> ArcMutex<BlockingSender<T>> {
        crate::common::arc::Arc::new(crate::common::blocking_mutex::BlockingMutex::new(BlockingSender {
            ptr: self.ptr,
            maker: PhantomData,
        }))
    }

    pub fn new_receiver(&self) -> ArcMutex<BlockingReceiver<T>> {
        crate::common::arc::Arc::new(crate::common::blocking_mutex::BlockingMutex::new(BlockingReceiver {
            ptr: self.ptr,
            maker: PhantomData,
        }))
    }
}

impl<T> BlockingSender<T> {
    pub fn send(&self, data: T) -> Result<(), i32> {
        unsafe {
            let data_ptr = &data as *const T as *const ();
            let id = rtos_msgq_send(self.ptr.as_ptr(), data_ptr, u32::MAX);
            if id == 0 {
                Ok(())
            } else {
                Err(id)
            }
        }
    }
}

impl<T: Default> BlockingReceiver<T> {
    fn recv_inner(&self, timeout: u32) -> Result<T, i32> {
        unsafe {
            let mut data = T::default();
            let data_ptr = &mut data as *mut T as *mut ();
            let id = rtos_msgq_recv(self.ptr.as_ptr(), data_ptr, timeout);
            if id == 0 {
                Ok(data)
            } else {
                Err(id)
            }
        }
    }

    pub fn recv(&self) -> Result<T, i32> {
        self.recv_inner(u32::MAX)
    }

    #[allow(dead_code)]
    pub fn try_recv(&self) -> Result<T, i32> {
        self.recv_inner(0)
    }
}

extern "C" {
    pub fn rtos_msgq_new(data_size: usize) -> *mut ();
    pub fn rtos_msgq_send(msgq: *mut (), data: *const (), timeout: u32) -> i32;
    pub fn rtos_msgq_recv(msgq: *mut (), data_out: *mut (), timeout: u32) -> i32;
}
