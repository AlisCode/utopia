use crate::{
    widgets::{TypedWidget, Widget},
    Backend,
};

pub mod click;

pub trait Controller<T, W: Widget<T>> {
    type Event;
    type Reaction;

    fn event(&mut self, child: &mut W, data: &mut T, event: &Self::Event)
        -> Option<Self::Reaction>;
}

impl<T, W: Widget<T>, C: Controller<T, W>> Controller<T, W> for Box<C> {
    type Event = C::Event;
    type Reaction = C::Reaction;

    fn event(
        &mut self,
        child: &mut W,
        data: &mut T,
        event: &Self::Event,
    ) -> Option<Self::Reaction> {
        self.as_mut().event(child, data, event)
    }
}

pub trait TypedController<T, W: TypedWidget<T, B>, B: Backend>:
    sealed::InnerTypedController<T, W, B>
{
    fn event(&mut self, child: &mut W, data: &mut T, event: &B::Event) -> Option<B::EventReaction>;
}

impl<T, W: TypedWidget<T, B>, B: Backend, C> TypedController<T, W, B> for C
where
    C: sealed::InnerTypedController<T, W, B>,
{
    fn event(&mut self, child: &mut W, data: &mut T, event: &B::Event) -> Option<B::EventReaction> {
        sealed::InnerTypedController::<T, W, B>::event(self, child, data, event)
    }
}

mod sealed {
    use super::{Controller, TransformEvent};
    use crate::{
        widgets::{TypedWidget, Widget},
        Backend,
    };

    pub trait InnerTypedController<T, W: TypedWidget<T, B>, B: Backend> {
        fn event(
            &mut self,
            child: &mut W,
            data: &mut T,
            event: &B::Event,
        ) -> Option<B::EventReaction>;
    }

    impl<T, W: TypedWidget<T, B> + Widget<T>, B: Backend, C> InnerTypedController<T, W, B> for C
    where
        C: Controller<T, W>,
        B::Event: TransformEvent<C::Event>,
        B::EventReaction: From<C::Reaction>,
    {
        fn event(
            &mut self,
            child: &mut W,
            data: &mut T,
            event: &B::Event,
        ) -> Option<B::EventReaction> {
            event.transform_event().and_then(|event| {
                Controller::event(self, child, data, event).map(|reaction| reaction.into())
            })
        }
    }
}

pub trait TransformEvent<Event> {
    fn transform_event(&self) -> Option<&Event>;
}

impl<T> TransformEvent<T> for T {
    fn transform_event(&self) -> Option<&T> {
        Some(self)
    }
}
