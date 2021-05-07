use crate::{controllers::TypedController, math::Size, BoxConstraints};
use crate::{math::Vector2, Backend};

use super::{TypedWidget, Widget};

pub struct Controlled<T, W, C, B: Backend> {
    widget: W,
    controller: C,
    _b: std::marker::PhantomData<B>,
    _t: std::marker::PhantomData<T>,
}

impl<T, W, C, B: Backend> Controlled<T, W, C, B> {
    pub fn new(widget: W, controller: C) -> Self {
        Controlled {
            widget,
            controller,
            _b: std::marker::PhantomData,
            _t: std::marker::PhantomData,
        }
    }
}

impl<T, W: TypedWidget<T, B> + Widget<T>, C: TypedController<T, B>, B: Backend> Widget<T>
    for Controlled<T, W, C, B>
{
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        TypedWidget::<T, B>::layout(&mut self.widget, bc, context, data)
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        TypedController::<T, B>::event(&mut self.controller, origin, size, data, event)
    }
}
