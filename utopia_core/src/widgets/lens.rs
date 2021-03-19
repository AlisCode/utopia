use crate::{
    lens::Lens,
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

use super::{TypedWidget, Widget};

pub struct LensWrap<T, U, L: Lens<T, U>, W: TypedWidget<U, B>, B: Backend> {
    lens: L,
    widget: W,
    _t: std::marker::PhantomData<T>,
    _u: std::marker::PhantomData<U>,
    _b: std::marker::PhantomData<B>,
}

impl<T, U, L: Lens<T, U>, W: TypedWidget<U, B>, B: Backend> std::ops::Deref
    for LensWrap<T, U, L, W, B>
{
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl<T, U, L: Lens<T, U>, W: TypedWidget<U, B>, B: Backend> std::ops::DerefMut
    for LensWrap<T, U, L, W, B>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

impl<T, U, L: Lens<T, U>, W: TypedWidget<U, B>, B: Backend> LensWrap<T, U, L, W, B> {
    pub fn new(widget: W, lens: L) -> Self {
        LensWrap {
            lens,
            widget,
            _t: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _b: std::marker::PhantomData,
        }
    }
}

impl<T, U, L: Lens<T, U>, W: TypedWidget<U, B>, B: Backend> Widget<T> for LensWrap<T, U, L, W, B> {
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let widget = &mut self.widget;
        self.lens.with(data, |data| {
            TypedWidget::<U, B>::layout(widget, bc, context, data)
        })
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        self.lens.with(data, |data| {
            TypedWidget::<U, B>::draw(&self.widget, origin, size, data)
        })
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        let widget = &mut self.widget;
        self.lens.with_mut(data, |data| {
            TypedWidget::<U, B>::event(widget, origin, size, data, event)
        })
    }
}
