use crate::{contexts::ContextProvider, math::Vector2};
use crate::{math::Size, BoxConstraints};

use super::Widget;

pub struct MapContext<W, T, C> {
    widget: W,
    _t: std::marker::PhantomData<T>,
    _lh: std::marker::PhantomData<C>,
}

impl<W: Widget<T>, T, C> MapContext<W, T, C> {
    pub fn new(widget: W) -> Self {
        MapContext {
            widget,
            _t: std::marker::PhantomData,
            _lh: std::marker::PhantomData,
        }
    }
}

impl<W: Widget<T>, T, C> Widget<T> for MapContext<W, T, C>
where
    C: ContextProvider<W::Context>,
{
    type Primitive = W::Primitive;
    type Context = C;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let context = <C as ContextProvider<W::Context>>::provide(&context);
        self.widget.layout(bc, context, data)
    }

    fn draw(&self, origin: Vector2, data: &T) -> Self::Primitive {
        self.widget.draw(origin, data)
    }
}
