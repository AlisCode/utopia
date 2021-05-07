use crate::math::{Size, Vector2};

use super::{Controller, TransformEvent};

pub struct OrController<CA, CB, E, R> {
    pub controller_a: CA,
    pub controller_b: CB,
    pub _event: std::marker::PhantomData<E>,
    pub _reaction: std::marker::PhantomData<R>,
}

impl<
        T,
        CA: Controller<T>,
        CB: Controller<T>,
        E: TransformEvent<CA::Event> + TransformEvent<CB::Event> + Clone,
        R: From<CA::Reaction> + From<CB::Reaction>,
    > Controller<T> for OrController<CA, CB, E, R>
{
    type Event = E;
    type Reaction = R;
    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: <Self as Controller<T>>::Event,
    ) -> Option<<Self as Controller<T>>::Reaction> {
        if let Some(event) = event.clone().transform_event() {
            return self
                .controller_a
                .event(origin, size, data, event)
                .map(|reaction| reaction.into());
        }

        if let Some(event) = event.transform_event() {
            return self
                .controller_b
                .event(origin, size, data, event)
                .map(|reaction| reaction.into());
        }

        None
    }
}
