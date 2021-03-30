use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AnimateEvent {
    pub elapsed: Duration,
}

impl AnimateEvent {
    pub fn new(elapsed: Duration) -> Self {
        AnimateEvent { elapsed }
    }
}
