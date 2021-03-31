use std::{future::Future, pin::Pin};

pub struct LaunchFuture<T, Data> {
    pub future: Pin<Box<dyn Future<Output = T>>>,
    pub callback: Box<dyn Fn(T, &mut Data)>,
}

impl<T, Data> LaunchFuture<T, Data> {
    pub fn new<F: Future<Output = T> + 'static, C: Fn(T, &mut Data) + 'static>(
        future: F,
        callback: C,
    ) -> Self {
        LaunchFuture {
            future: Box::pin(future),
            callback: Box::new(callback),
        }
    }
}
