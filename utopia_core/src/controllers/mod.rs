use crate::{
    math::{Size, Vector2},
    Backend,
};

use self::{map::MapReaction, or::OrController};

pub mod click;
pub mod map;
pub mod or;

/// A Controller is a bit of logic that reacts to a specific Event type.
pub trait Controller<T> {
    type Event;
    type Reaction;

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction>;
}

pub trait ControllerExt<T>: Controller<T> + Sized {
    fn map<R, F: Fn(Self::Reaction) -> R + 'static>(self, mapper: F) -> MapReaction<Self, F> {
        MapReaction {
            controller: self,
            mapper,
        }
    }

    fn or<C: Controller<T>, E, R>(self, controller: C) -> OrController<Self, C, E, R> {
        OrController {
            controller_a: self,
            controller_b: controller,
            _event: std::marker::PhantomData,
            _reaction: std::marker::PhantomData,
        }
    }
}

impl<T, C: Controller<T>> ControllerExt<T> for C {}

impl<T, C: Controller<T>> Controller<T> for Box<C> {
    type Event = C::Event;
    type Reaction = C::Reaction;

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.as_mut().event(origin, size, data, event)
    }
}

pub trait TypedController<T, B: Backend>: sealed::InnerTypedController<T, B> {
    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: B::Event,
    ) -> Option<B::EventReaction>;
}

impl<T, B: Backend, C> TypedController<T, B> for C
where
    C: sealed::InnerTypedController<T, B>,
{
    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: B::Event,
    ) -> Option<B::EventReaction> {
        sealed::InnerTypedController::<T, B>::event(self, origin, size, data, event)
    }
}

mod sealed {
    use super::{Controller, TransformEvent};
    use crate::{
        math::{Size, Vector2},
        Backend,
    };

    pub trait InnerTypedController<T, B: Backend> {
        fn event(
            &mut self,
            origin: Vector2,
            size: Size,
            data: &mut T,
            event: B::Event,
        ) -> Option<B::EventReaction>;
    }

    impl<T, B: Backend, C> InnerTypedController<T, B> for C
    where
        C: Controller<T>,
        B::Event: TransformEvent<C::Event>,
        B::EventReaction: From<C::Reaction>,
    {
        fn event(
            &mut self,
            origin: Vector2,
            size: Size,
            data: &mut T,
            event: B::Event,
        ) -> Option<B::EventReaction> {
            event.transform_event().and_then(|event| {
                Controller::event(self, origin, size, data, event).map(|reaction| reaction.into())
            })
        }
    }
}

pub trait TransformEvent<Event> {
    fn transform_event(self) -> Option<Event>;
}

impl<T> TransformEvent<T> for T {
    fn transform_event(self) -> Option<T> {
        Some(self)
    }
}

pub enum Or<A, B> {
    A(A),
    B(B),
}
