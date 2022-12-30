use core::future::Future;
use core::marker::PhantomData;
use core::ptr::NonNull;
use crate::zbus::backend::{ZBusNotifier, ZBusPublisher, ZBusReader};
use crate::zbus::channel::ZbusChannel;
use crate::zbus::no_std::{CZbusChannel, CZbusObserver, name_convert};

pub struct ZBusSubscriber<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> {
    name: &'static str,
    c_ref: NonNull<CZbusObserver>,
    marker: PhantomData<T>,
    marker_r: PhantomData<R>,
    marker_p: PhantomData<P>,
    marker_n: PhantomData<N>,
    backend: Backend,
}

impl<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> ZBusSubscriber<T, R, P, N, Backend> {
    pub fn load(c_ref: *const CZbusObserver) -> Self {
        let c_ref = unsafe { NonNull::new_unchecked(c_ref as *mut CZbusObserver) };
        ZBusSubscriber {
            name: unsafe { name_convert(c_ref.as_ref().name) },
            c_ref,
            marker: PhantomData,
            marker_r: PhantomData,
            marker_p: PhantomData,
            marker_n: PhantomData,
            backend: Backend::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn enable(&mut self) {
        unsafe {
            self.c_ref.as_mut().enabled = true;
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            self.c_ref.as_mut().enabled = false;
        }
    }

    pub async fn wait(&self) -> ZbusChannel<T, R, P, N, Backend> {
        todo!()
    }
}

pub struct ZBusListener<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> {
    name: &'static str,
    c_ref: NonNull<CZbusObserver>,
    marker: PhantomData<T>,
    marker_r: PhantomData<R>,
    marker_p: PhantomData<P>,
    marker_n: PhantomData<N>,
    backend: Backend,
    callback: fn(chan: *const CZbusChannel),
}

impl<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> ZBusListener<T, R, P, N, Backend> {
    pub fn load(c_ref: *const CZbusObserver, callback: fn(chan: *const CZbusChannel)) -> Self {
        let c_ref = unsafe { NonNull::new_unchecked(c_ref as *mut CZbusObserver) };
        let listener = ZBusListener {
            name: unsafe { name_convert(c_ref.as_ref().name) },
            c_ref,
            marker: PhantomData,
            marker_r: PhantomData,
            marker_p: PhantomData,
            marker_n: PhantomData,
            backend: Backend::default(),
            callback,
        };

        todo!();

        listener
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn enable(&mut self) {
        unsafe {
            self.c_ref.as_mut().enabled = true;
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            self.c_ref.as_mut().enabled = false;
        }
    }
}
