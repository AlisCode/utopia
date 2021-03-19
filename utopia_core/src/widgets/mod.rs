use std::ops::{Deref, DerefMut};

use controlled::Controlled;
use lens::LensWrap;

use crate::{
    controllers::TypedController,
    lens::Lens,
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

use self::styled::Styled;

pub mod controlled;
pub mod lens;
pub mod pod;
pub mod styled;

pub trait CoreExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn controlled<C: TypedController<T, Self, B>>(
        self,
        controller: C,
    ) -> Controlled<T, Self, C, B> {
        Controlled::new(self, controller)
    }

    fn lens<U, L: Lens<T, U>>(self, lens: L) -> LensWrap<T, U, L, Self, B>
    where
        Self: TypedWidget<U, B>,
    {
        LensWrap::new(self, lens)
    }

    fn styled<U: Clone, W: TypedWidget<T, B>, L: Lens<T, U>, LW: Lens<W, U>>(
        self,
        lens: L,
        lens_widget: LW,
    ) -> Styled<U, L, LW, W, Self, B>
    where
        Self: Deref<Target = W> + DerefMut,
    {
        Styled::new(self, lens, lens_widget)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + 'static> CoreExt<T, B> for W {}

pub trait Widget<T> {
    type Primitive;
    type Context;
    type Event;
    type Reaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive;
    fn event(
        &mut self,
        _origin: Vector2,
        _size: Size,
        _data: &mut T,
        _event: Self::Event,
    ) -> Option<Self::Reaction> {
        None
    }
    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size;
}

impl<T> Widget<T> for () {
    type Primitive = ();
    type Context = ();
    type Event = ();
    type Reaction = ();

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
    type Event = W::Event;
    type Reaction = W::Reaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        self.as_mut().layout(bc, context, data)
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.as_mut().event(origin, size, data, event)
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        self.as_ref().draw(origin, size, data)
    }
}

pub trait TypedWidget<T, B: Backend>: sealed::InnerTypedWidget<T, B> {
    fn draw(&self, origin: Vector2, size: Size, data: &T) -> B::Primitive;
    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: B::Event,
    ) -> Option<B::EventReaction>;
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
}

impl<T, B: Backend, TW> TypedWidget<T, B> for TW
where
    TW: sealed::InnerTypedWidget<T, B>,
{
    fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size {
        <Self as sealed::InnerTypedWidget<T, B>>::layout(self, bc, backend, data)
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: B::Event,
    ) -> Option<B::EventReaction> {
        <Self as sealed::InnerTypedWidget<T, B>>::event(self, origin, size, data, event)
    }

    fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive {
        <Self as sealed::InnerTypedWidget<T, B>>::draw(self, bounds, size, data)
    }
}

mod sealed {
    use super::Widget;
    use crate::{
        contexts::ContextProvider, controllers::TransformEvent, math::Vector2, Backend,
        BoxConstraints, Size,
    };

    pub trait InnerTypedWidget<T, B: Backend> {
        fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive;
        fn event(
            &mut self,
            origin: Vector2,
            size: Size,
            data: &mut T,
            event: B::Event,
        ) -> Option<B::EventReaction>;
        fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size;
    }

    impl<T, W, P, C, B: Backend> InnerTypedWidget<T, B> for W
    where
        W: Widget<T, Primitive = P, Context = C>,
        B::Primitive: From<P>,
        B: ContextProvider<C>,
        B::Event: TransformEvent<W::Event>,
        B::EventReaction: From<W::Reaction>,
    {
        fn layout(&mut self, bc: &BoxConstraints, backend: &B, data: &T) -> Size {
            let context = backend.provide();
            <Self as Widget<T>>::layout(self, bc, context, data)
        }

        fn event(
            &mut self,
            origin: Vector2,
            size: Size,
            data: &mut T,
            event: B::Event,
        ) -> Option<B::EventReaction> {
            event.transform_event().and_then(|event| {
                <Self as Widget<T>>::event(self, origin, size, data, event)
                    .map(|reaction| reaction.into())
            })
        }

        fn draw(&self, bounds: Vector2, size: Size, data: &T) -> B::Primitive {
            <Self as Widget<T>>::draw(self, bounds, size, data).into()
        }
    }
}
