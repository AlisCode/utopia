use crate::{
    lens::Lens,
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

use super::{pod::WidgetPod, TypedWidget, Widget};

pub struct LensWrap<T, U, L: Lens<T, U>, B: Backend> {
    lens: L,
    widget: WidgetPod<U, B>,
    _t: std::marker::PhantomData<T>,
}

impl<T, U, L: Lens<T, U>, B: Backend> LensWrap<T, U, L, B> {
    pub fn new<TW: TypedWidget<U, B> + 'static>(widget: TW, lens: L) -> Self {
        LensWrap {
            lens,
            widget: WidgetPod::new(widget),
            _t: std::marker::PhantomData,
        }
    }
}

impl<T, U, L: Lens<T, U>, B: Backend> Widget<T> for LensWrap<T, U, L, B> {
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
}
