use core::future::Future;
use core::marker::PhantomData;
use core::ptr::{NonNull};
use crate::zbus::backend::{ZBusNotifier, ZBusPublisher, ZBusReader};
use crate::zbus::error::ZbusError;
use crate::zbus::no_std::{CZbusChannel, name_convert};
use crate::zbus::observers::{ZBusListener, ZBusSubscriber};

pub struct ZbusChannel<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> {
    name: &'static str,
    c_ref: NonNull<CZbusChannel>,
    marker_t: PhantomData<T>,
    marker_r: PhantomData<R>,
    marker_p: PhantomData<P>,
    marker_n: PhantomData<N>,
    backend: Backend,
}

impl<T, R: Future<Output=T>, P: Future<Output=()>, N: Future<Output=()>, Backend: Default + ZBusReader<T, R> + ZBusPublisher<T, P> + ZBusNotifier<N>> ZbusChannel<T, R, P, N, Backend> {
    pub fn load(c_ref: *const CZbusChannel) -> Self {
        // TODO: Explain because this is unsafe
        let c_ref = unsafe { NonNull::new_unchecked(c_ref as *mut CZbusChannel) };
        Self {
            // TODO: Explain because this is unsafe
            name: unsafe { name_convert(c_ref.as_ref().name) },
            c_ref,
            marker_t: PhantomData,
            marker_r: PhantomData,
            marker_p: PhantomData,
            marker_n: PhantomData,
            backend: Backend::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn user_data<U>(&self) -> Option<&U> {
        let user_data = unsafe { self.c_ref.as_ref().user_data };
        if user_data.is_null() {
            None
        } else {
            unsafe {
                Some(&*(user_data as *const () as *const U))
            }
        }
    }

    pub fn mut_user_data<U>(&mut self) -> Option<&mut U> {
        let user_data = unsafe { self.c_ref.as_mut().user_data };
        if user_data.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *(user_data as *mut U))
            }
        }
    }

    pub fn set_user_data<U>(&mut self, data: &mut U) {
        unsafe { self.c_ref.as_mut().user_data = data as *mut U as *mut () }
    }

    pub async fn add_subscriber(&mut self, observer: ZBusSubscriber<T, R, P, N, Backend>) -> Result<(), ZbusError> {
        todo!()
    }

    pub async fn add_listener(&mut self, listener: ZBusListener<T, R, P, N, Backend>) -> Result<(), ZbusError> {
        todo!()
    }

    pub async fn remove_subscriber(&mut self, observer: ZBusSubscriber<T, R, P, N, Backend>) -> Result<(), ZbusError> {
        todo!()
    }

    pub async fn remove_listener(&mut self, listener: ZBusListener<T, R, P, N, Backend>) -> Result<(), ZbusError> {
        todo!()
    }

    pub async fn read(&self) -> T {
        self.backend.reader().await
    }

    pub async fn publish(&mut self, data: T) {
        self.backend.publisher(data).await
    }

    pub async fn notify(&mut self) {
        self.backend.notifier().await
    }
}
