use crate::math::{Size, Vector2};

use super::Controller;

pub struct MapReaction<C, F> {
    pub(crate) controller: C,
    pub(crate) mapper: F,
}

impl<T, R, C: Controller<T>, F: Fn(C::Reaction) -> R> Controller<T> for MapReaction<C, F> {
    type Event = C::Event;
    type Reaction = R;

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.controller
            .event(origin, size, data, event)
            .map(&self.mapper)
    }
}
