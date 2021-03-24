use crate::{
    math::{Size, Vector2},
    widgets::TypedWidget,
    Backend,
};

pub struct EventStep<E, R> {
    pub size: Size,
    event_queue: Vec<E>,
    reaction_queue: Vec<R>,
}

impl<E, R> Default for EventStep<E, R> {
    fn default() -> Self {
        EventStep {
            size: Size::default(),
            event_queue: Vec::default(),
            reaction_queue: Vec::default(),
        }
    }
}

impl<E, R> EventStep<E, R> {
    pub fn queue_event(&mut self, event: E) {
        self.event_queue.push(event)
    }
}

impl<E, R> EventStep<E, R> {
    pub fn apply<T, B, TW: TypedWidget<T, B>>(&mut self, visitable: &mut TW, data: &mut T)
    where
        B: Backend<Event = E, EventReaction = R>,
    {
        let size = self.size;
        let mut reactions: Vec<B::EventReaction> = self
            .event_queue
            .drain(0..)
            .filter_map(|event| {
                <TW as TypedWidget<T, B>>::event(visitable, Vector2::ZERO, size, data, event)
            })
            .collect();
        self.reaction_queue.append(&mut reactions);
    }
}
