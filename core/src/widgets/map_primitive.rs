use crate::{
    math::{Size, Vector2},
    BoxConstraints,
};

use super::Widget;

pub struct MapPrimitive<W: Widget<T>, T, P> {
    widget: W,
    mapper: fn(W::Primitive) -> P,
}

impl<W: Widget<T>, T, P> MapPrimitive<W, T, P> {
    pub fn new(widget: W) -> Self
    where
        P: From<W::Primitive>,
    {
        MapPrimitive {
            widget,
            mapper: <P as From<W::Primitive>>::from,
        }
    }

    pub fn new_with_mapper(widget: W, mapper: fn(W::Primitive) -> P) -> Self {
        MapPrimitive { widget, mapper }
    }
}

impl<W: Widget<T>, T, P> Widget<T> for MapPrimitive<W, T, P> {
    type Primitive = P;
    type Context = W::Context;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        self.widget.layout(bc, &context, data)
    }

    fn draw(&self, origin: Vector2, data: &T) -> Self::Primitive {
        let primitive = self.widget.draw(origin, data);
        (self.mapper)(primitive)
    }
}
