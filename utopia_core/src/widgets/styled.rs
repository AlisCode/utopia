use std::ops::{Deref, DerefMut};

use crate::{
    lens::Lens,
    math::{Size, Vector2},
    Backend,
};

use super::{TypedWidget, Widget};

pub struct Styled<U, L, LW, W, TW, B> {
    widget: TW,
    lens: L,
    lens_widget: LW,
    _w: std::marker::PhantomData<W>,
    _u: std::marker::PhantomData<U>,
    _b: std::marker::PhantomData<B>,
}

impl<U: Clone, L, LW: Lens<W, U>, W, TW: Deref<Target = W> + DerefMut, B: Backend>
    Styled<U, L, LW, W, TW, B>
{
    pub fn new<T>(widget: TW, lens: L, lens_widget: LW) -> Self
    where
        L: Lens<T, U>,
    {
        Styled {
            widget: widget,
            lens,
            lens_widget,
            _w: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _b: std::marker::PhantomData,
        }
    }
}

impl<
        T,
        U: Clone,
        W,
        TW: Deref<Target = W> + DerefMut + TypedWidget<T, B>,
        L: Lens<T, U>,
        LW: Lens<W, U>,
        B: Backend,
    > Widget<T> for Styled<U, L, LW, W, TW, B>
{
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn layout(&mut self, bc: &crate::BoxConstraints, context: &Self::Context, data: &T) -> Size {
        // TODO: This should probably move to a "lifecycle" handling function ?
        let styled_prop = self.lens.with(data, |data| data.clone());
        let widget = &mut self.widget;
        self.lens_widget.with_mut(widget, |widget_data| {
            *widget_data = styled_prop;
        });
        TypedWidget::<T, B>::layout(widget, bc, context, data)
    }
}
