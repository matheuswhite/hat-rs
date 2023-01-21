extern crate alloc;

use core::ptr::NonNull;
use core::marker::PhantomData;
use core::ops::Deref;
use core::mem;
use core::sync::atomic;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::alloc::Layout;
use alloc::boxed::Box;
use crate::common::result::Expect;

pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
}

pub struct ArcInner<T> {
    data: T,
    rc: AtomicUsize,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        let boxed = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });
        Arc {
            ptr: NonNull::new(Box::into_raw(boxed)).hat_expect("The pointer is non-null"),
            phantom: PhantomData,
        }
    }

    pub fn into_raw(this: Self) -> *const T {
        let ptr_inner = NonNull::as_ptr((&this).ptr);
        let ptr_data = unsafe { core::ptr::addr_of_mut!((*ptr_inner).data) };
        mem::forget(this);
        ptr_data
    }

    pub unsafe fn from_raw(ptr: *const T) -> Self {
        let ptr_inner = ptr as *mut ArcInner<T>;

        Arc {
            ptr: NonNull::new_unchecked(ptr_inner),
            phantom: PhantomData,
        }
    }

    fn set_ptr_value(mut this: *mut T, val: *mut u8) -> *mut T {
        let thin = &mut this as *mut *mut T as *mut *mut u8;
        unsafe { *thin = val };
        this
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };

        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc >= isize::MAX as usize {
            panic!("arc inc error");
        }

        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        atomic::fence(Ordering::Acquire);

        unsafe { Box::from_raw(self.ptr.as_ptr()); }
    }
}

const fn padding_needed_for(this: &Layout, align: usize) -> usize {
    let len = this.size();
    let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
    len_rounded_up.wrapping_sub(len)
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}

unsafe impl<T: Sync + Send> Sync for Arc<T> {}
