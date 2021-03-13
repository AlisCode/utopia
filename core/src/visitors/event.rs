use crate::{
    math::{Size, Vector2},
    widgets::TypedWidget,
    Backend,
};

use super::VisitorMut;

pub struct EventVisitor<E> {
    pub size: Size,
    event_queue: Vec<E>,
}

impl<E> Default for EventVisitor<E> {
    fn default() -> Self {
        EventVisitor {
            size: Size::default(),
            event_queue: Vec::default(),
        }
    }
}

impl<E> EventVisitor<E> {
    pub fn queue_event(&mut self, event: E) {
        self.event_queue.push(event)
    }
}

impl<T, B: Backend> VisitorMut<T, B> for EventVisitor<B::Event> {
    type Output = ();

    fn visit_mut<V: TypedWidget<T, B>>(&mut self, visitable: &mut V, _backend: &B, data: &mut T) {
        let size = self.size;
        let _reactions: Vec<_> = self
            .event_queue
            .drain(0..)
            .map(|event| {
                <V as TypedWidget<T, B>>::event(visitable, Vector2::ZERO, size, data, event);
            })
            .collect();
    }

    fn finish(self) -> Self::Output {
        todo!()
    }
}
