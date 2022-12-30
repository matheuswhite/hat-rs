use core::future::Future;

pub trait ZBusReader<T, B: Future<Output=T>> {
    fn reader(&self) -> B;
}

pub trait ZBusPublisher<T, B: Future<Output=()>> {
    fn publisher(&self, data: T) -> B;
}

pub trait ZBusNotifier<B: Future<Output=()>> {
    fn notifier(&self) -> B;
}
