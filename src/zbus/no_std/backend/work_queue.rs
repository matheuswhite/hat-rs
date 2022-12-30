use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use crate::zbus::no_std::CZbusChannel;
use super::super::super::backend::{ZBusReader, ZBusPublisher, ZBusNotifier};
use super::super::super::channel::ZbusChannel;

type ZBusChannelWorkQueue<T> = ZbusChannel<T, ZBusReaderWorkQueue<T>, ZBusPublisherWorkQueue<T>, ZBusNotifierWorkQueue, ZBusBackendWorkQueue<T>>;

// Example
async fn foo() {
    let c_channel = 0x1000 as *const CZbusChannel;
    let mut chan = ZBusChannelWorkQueue::<bool>::load(c_channel);

    let data = chan.read().await;
    chan.publish(data).await;
}

pub struct ZBusBackendWorkQueue<T> {
    marker_t: PhantomData<T>,
}

impl<T> Default for ZBusBackendWorkQueue<T> {
    fn default() -> Self {
        Self {
            marker_t: PhantomData,
        }
    }
}

impl<T> ZBusReader<T, ZBusReaderWorkQueue<T>> for ZBusBackendWorkQueue<T> {
    fn reader(&self) -> ZBusReaderWorkQueue<T> {
        ZBusReaderWorkQueue::default()
    }
}

impl<T> ZBusPublisher<T, ZBusPublisherWorkQueue<T>> for ZBusBackendWorkQueue<T> {
    fn publisher(&self, data: T) -> ZBusPublisherWorkQueue<T> {
        ZBusPublisherWorkQueue::new(data)
    }
}

impl<T> ZBusNotifier<ZBusNotifierWorkQueue> for ZBusBackendWorkQueue<T> {
    fn notifier(&self) -> ZBusNotifierWorkQueue {
        ZBusNotifierWorkQueue::default()
    }
}

// Reader
pub struct ZBusReaderWorkQueue<T> {
    marker_t: PhantomData<T>,
}

impl<T> Default for ZBusReaderWorkQueue<T> {
    fn default() -> Self {
        Self {
            marker_t: PhantomData
        }
    }
}

impl<T> Future for ZBusReaderWorkQueue<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

// Publisher
pub struct ZBusPublisherWorkQueue<T> {
    marker_t: PhantomData<T>,
    data: T,
}

impl<T> ZBusPublisherWorkQueue<T> {
    fn new(data: T) -> Self {
        Self {
            marker_t: PhantomData,
            data,
        }
    }
}

impl<T> Future for ZBusPublisherWorkQueue<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

// Notifier
pub struct ZBusNotifierWorkQueue {}

impl Default for ZBusNotifierWorkQueue {
    fn default() -> Self {
        Self {}
    }
}

impl Future for ZBusNotifierWorkQueue {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
