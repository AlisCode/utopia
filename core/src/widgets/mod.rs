use border::Border;

use crate::{
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

pub mod border;
pub mod flex;
pub mod lens;
pub mod padding;
pub mod pod;
pub mod text;

pub trait WidgetExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn bordered<Color: Default>(self) -> Border<T, Color, B> {
        Border::new(self)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + 'static> WidgetExt<T, B> for W {}

pub trait Widget<T> {
    type Primitive;
    type Context;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size;
    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive;
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

    fn draw(&self, _origin: Vector2, _size: Size, _data: &T) -> Self::Primitive {}
}

impl<T, W: Widget<T>> Widget<T> for Box<W> {
    type Primitive = W::Primitive;
    type Context = W::Context;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        self.as_mut().layout(bc, context, data)
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        self.as_ref().draw(origin, size, data)
    }
}

pub trait TypedWidget<T, B: Backend>: sealed::InnerTypedWidget<T, B> {
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
    fn draw(&self, origin: Vector2, size: Size, data: &T) -> B::Primitive;
}

impl<T, B: Backend, TW> TypedWidget<T, B> for TW
where
    TW: sealed::InnerTypedWidget<T, B>,
{
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size {
        <Self as sealed::InnerTypedWidget<T, B>>::layout(self, bc, backend, data)
    }

    fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive {
        <Self as sealed::InnerTypedWidget<T, B>>::draw(self, bounds, size, data)
    }
}

mod sealed {
    use super::Widget;
    use crate::{contexts::ContextProvider, math::Vector2, Backend, BoxConstraints, Size};

    pub trait InnerTypedWidget<T, B: Backend> {
        fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
        fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive;
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

        fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive {
            <Self as Widget<T>>::draw(self, bounds, size, data).into()
        }
    }
}
