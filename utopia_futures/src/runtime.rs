use std::sync::{Arc, Mutex};

use crate::{controller::AsyncResolution, executor::Executor, reaction::LaunchFuture};
use futures::FutureExt;

pub struct AsyncRuntime<E> {
    executor: E,
    async_responses: Arc<Mutex<Vec<AsyncResolution>>>,
}

impl<E: Executor> AsyncRuntime<E> {
    pub fn new(executor: E) -> Self {
        AsyncRuntime {
            executor,
            async_responses: Arc::new(Mutex::new(Vec::default())),
        }
    }

    pub fn handle_reaction<T: 'static>(&mut self, launch: LaunchFuture<T>) {
        let LaunchFuture { future } = launch;

        let responses = self.async_responses.clone();
        let future = future.then(|output| async move {
            let mut responses = responses
                .lock()
                .expect("Failed to get lock on async responses");
            responses.push(AsyncResolution::new(output));
        });
        self.executor.spawn(future)
    }
}
