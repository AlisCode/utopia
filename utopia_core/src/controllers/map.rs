use crate::{
    math::{Size, Vector2},
    widgets::Widget,
};

use super::Controller;

pub struct MapReaction<C, F> {
    pub(crate) controller: C,
    pub(crate) mapper: F,
}

impl<T, R, W: Widget<T>, C: Controller<T, W>, F: Fn(C::Reaction) -> R> Controller<T, W>
    for MapReaction<C, F>
{
    type Event = C::Event;
    type Reaction = R;

    fn event(
        &mut self,
        child: &mut W,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.controller
            .event(child, origin, size, data, event)
            .map(&self.mapper)
    }
}
