use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError, TryRecvError};
use crate::common::ArcMutex;
use crate::common::arc::Arc;
use crate::common::blocking_mutex::BlockingMutex;

pub struct BlockingChannel<T: Clone> {
    sender: ArcMutex<BlockingSender<T>>,
    receiver: ArcMutex<BlockingReceiver<T>>,
}

impl<T: Clone> BlockingChannel<T> {
    pub fn new() -> Self {
        let (s, r) = channel();

        BlockingChannel {
            sender: Arc::new(BlockingMutex::new(BlockingSender::new(s))),
            receiver: Arc::new(BlockingMutex::new(BlockingReceiver::new(r))),
        }
    }

    pub fn new_sender(&self) -> ArcMutex<BlockingSender<T>> {
        self.sender.clone()
    }

    pub fn new_receiver(&self) -> ArcMutex<BlockingReceiver<T>> {
        self.receiver.clone()
    }
}

pub struct BlockingReceiver<T> {
    receiver: Receiver<T>,
}

impl<T> BlockingReceiver<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self {
            receiver
        }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}

#[derive(Clone)]
pub struct BlockingSender<T: Clone> {
    sender: Sender<T>,
}

impl<T: Clone> BlockingSender<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self {
            sender,
        }
    }

    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        self.sender.send(data)
    }
}
