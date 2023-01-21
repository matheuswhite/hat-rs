use core::ptr::NonNull;
use crate::zbus::channel::{Backend, BackendContructor, struct_zbus_channel};
use crate::zbus::no_std::work_queue_backend::claimer::WorkQueueClaimerFuture;
use crate::zbus::no_std::work_queue_backend::notifier::WorkQueueNotifierFuture;
use crate::zbus::no_std::work_queue_backend::publisher::WorkQueuePublisherFuture;
use crate::zbus::no_std::work_queue_backend::reader::WorkQueueReaderFuture;
use crate::zbus::no_std::work_queue_backend::waiter::WorkQueueWaiterFuture;

pub struct WorkQueueBackend {
    channel_ref: NonNull<struct_zbus_channel>,
}

impl BackendContructor for WorkQueueBackend {
    fn new(channel_ref: NonNull<struct_zbus_channel>) -> Self {
        Self {
            channel_ref,
        }
    }
}

impl<T: Default> Backend<T> for WorkQueueBackend {
    async fn read(&self) -> T {
        WorkQueueReaderFuture::new(self.channel_ref).await
    }

    async fn publish(&self, data: T) {
        WorkQueuePublisherFuture::new(self.channel_ref, data).await
    }

    async fn notify(&self) {
        WorkQueueNotifierFuture::new(self.channel_ref).await
    }

    async fn claim(&self) {
        WorkQueueClaimerFuture::new(self.channel_ref).await
    }

    async fn wait_msg(&self) -> T {
        WorkQueueWaiterFuture::new(self.channel_ref).await
    }
}
