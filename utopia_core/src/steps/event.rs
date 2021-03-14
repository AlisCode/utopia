use crate::{
    math::{Size, Vector2},
    widgets::TypedWidget,
    Backend,
};

pub struct EventStep<E> {
    pub size: Size,
    event_queue: Vec<E>,
}

impl<E> Default for EventStep<E> {
    fn default() -> Self {
        EventStep {
            size: Size::default(),
            event_queue: Vec::default(),
        }
    }
}

impl<E> EventStep<E> {
    pub fn queue_event(&mut self, event: E) {
        self.event_queue.push(event)
    }
}

impl<E> EventStep<E> {
    pub fn apply<T, B, TW: TypedWidget<T, B>>(&mut self, visitable: &mut TW, data: &mut T)
    where
        B: Backend<Event = E>,
    {
        let size = self.size;
        let _reactions: Vec<_> = self
            .event_queue
            .drain(0..)
            .map(|event| {
                <TW as TypedWidget<T, B>>::event(visitable, Vector2::ZERO, size, data, event);
            })
            .collect();
    }
}
