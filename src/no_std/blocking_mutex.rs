use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

pub struct BlockingMutex<T> {
    ptr: NonNull<()>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: 'a> {
    lock: &'a BlockingMutex<T>,
}

impl<T> BlockingMutex<T> {
    pub fn new(data: T) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(rtos_mutex_new()) };

        Self {
            ptr,
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, T>, i32> {
        unsafe {
            let id = rtos_mutex_lock(self.ptr.as_ptr(), u32::MAX);
            if id == 0 {
                Ok(MutexGuard::new(self))
            } else {
                Err(id)
            }
        }
    }
}

impl<'mutex, T> MutexGuard<'mutex, T> {
    pub fn new(lock: &'mutex BlockingMutex<T>) -> Self {
        Self {
            lock
        }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.lock.data.get()
        }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.lock.data.get()
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let id = rtos_mutex_unlock(self.lock.ptr.as_ptr());
            if id != 0 {
                panic!("Error at mutex unlock");
            }
        }
    }
}

impl<T> Drop for BlockingMutex<T> {
    fn drop(&mut self) {
        unsafe { rtos_mutex_del(self.ptr.as_ptr()); }
    }
}

extern "C" {
    pub fn rtos_mutex_new() -> *mut ();
    pub fn rtos_mutex_del(mutex: *mut ());
    pub fn rtos_mutex_lock(mutex: *mut (), timeout: u32) -> i32;
    pub fn rtos_mutex_unlock(mutex: *mut ()) -> i32;
}
