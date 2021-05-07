use std::future::Future;

pub trait Executor: Default {
    fn spawn<F: Future<Output = ()>>(&mut self, future: F);
}
