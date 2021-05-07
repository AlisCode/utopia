use std::{future::Future, pin::Pin};

pub struct LaunchFuture<T> {
    pub future: Pin<Box<dyn Future<Output = T>>>,
}

impl<T> LaunchFuture<T> {
    pub fn new<F: Future<Output = T> + 'static>(future: F) -> Self {
        LaunchFuture {
            future: Box::pin(future),
        }
    }
}
