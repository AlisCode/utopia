use crate::{widgets::Widget, Backend};

pub trait Controller<T, W: Widget<T>> {
    type Event;
    type Reaction;

    fn event(&mut self, child: &mut W, data: &mut T, event: &Self::Event)
        -> Option<Self::Reaction>;
}

pub trait TypedController<T, W: Widget<T>, B: Backend>:
    sealed::InnerTypedController<T, W, B>
{
    fn event(&mut self, child: &mut W, data: &mut T, event: &B::Event) -> Option<B::EventReaction>;
}

impl<T, W: Widget<T>, B: Backend, C> TypedController<T, W, B> for C
where
    C: sealed::InnerTypedController<T, W, B>,
{
    fn event(&mut self, child: &mut W, data: &mut T, event: &B::Event) -> Option<B::EventReaction> {
        sealed::InnerTypedController::<T, W, B>::event(self, child, data, event)
    }
}

mod sealed {
    use super::{Controller, TransformEvent};
    use crate::{widgets::Widget, Backend};

    pub trait InnerTypedController<T, W: Widget<T>, B: Backend> {
        fn event(
            &mut self,
            child: &mut W,
            data: &mut T,
            event: &B::Event,
        ) -> Option<B::EventReaction>;
    }

    impl<T, W: Widget<T>, B: Backend, C> InnerTypedController<T, W, B> for C
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
