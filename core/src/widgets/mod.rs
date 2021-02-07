use map_context::MapContext;
use map_primitive::MapPrimitive;

use crate::{
    contexts::ContextProvider,
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

pub mod border;
pub mod flex;
pub mod map_context;
pub mod map_primitive;
pub mod pod;
pub mod text;

pub trait WidgetExt<T>: Widget<T> + Sized {
    fn map_primitive<P: From<Self::Primitive>>(self) -> MapPrimitive<Self, T, P> {
        MapPrimitive::new(self)
    }

    fn map_context<C: ContextProvider<Self::Context>>(self) -> MapContext<Self, T, C> {
        MapContext::new(self)
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<W: Widget<T>, T> WidgetExt<T> for W {}

pub trait Widget<T> {
    type Primitive;
    type Context;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size;
    fn draw(&self, bounds: Vector2, data: &T) -> Self::Primitive;
}

impl<T> Widget<T> for () {
    type Primitive = ();
    type Context = ();

    fn layout(&mut self, _bc: &BoxConstraints, _context: &Self::Context, _data: &T) -> Size {
        Size {
            width: 0.,
            height: 0.,
        }
    }

    fn draw(&self, _bounds: Vector2, _data: &T) -> Self::Primitive {}
}

pub trait TypedWidget<T, B: Backend>: sealed::InnerTypedWidget<T, B> {
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
    fn draw(&self, origin: Vector2, data: &T) -> B::Primitive;
}

impl<T, B: Backend, TW> TypedWidget<T, B> for TW
where
    TW: sealed::InnerTypedWidget<T, B>,
{
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size {
        <Self as sealed::InnerTypedWidget<T, B>>::layout(self, bc, backend, data)
    }

    fn draw(&self, bounds: Vector2, data: &T) -> B::Primitive {
        <Self as sealed::InnerTypedWidget<T, B>>::draw(self, bounds, data)
    }
}

mod sealed {
    use super::Widget;
    use crate::{contexts::ContextProvider, math::Vector2, Backend, BoxConstraints, Size};

    pub trait InnerTypedWidget<T, B: Backend> {
        fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
        fn draw(&self, bounds: Vector2, data: &T) -> B::Primitive;
    }

    impl<T, W, P, C, B: Backend> InnerTypedWidget<T, B> for W
    where
        W: Widget<T, Primitive = P, Context = C>,
        B::Primitive: From<P>,
        B: ContextProvider<C>,
    {
        fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size {
            let context = backend.provide();
            <Self as Widget<T>>::layout(self, bc, context, data)
        }

        fn draw(&self, bounds: Vector2, data: &T) -> B::Primitive {
            <Self as Widget<T>>::draw(self, bounds, data).into()
        }
    }
}
