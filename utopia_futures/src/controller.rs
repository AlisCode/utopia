use std::any::Any;

use utopia_core::{
    controllers::{or::OrController, Controller, ControllerExt, TypedController},
    math::{Size, Vector2},
    Backend,
};

pub struct AsyncResolveController<T, FT> {
    callback: Box<dyn Fn(&mut T, FT)>,
}

pub struct AsyncResolution {
    output: Box<dyn Any>,
}

impl AsyncResolution {
    pub fn new<T: 'static>(output: T) -> Self {
        AsyncResolution {
            output: Box::new(output),
        }
    }
}

impl<T, FT: 'static> Controller<T> for AsyncResolveController<T, FT> {
    type Event = AsyncResolution;
    type Reaction = ();

    fn event(
        &mut self,
        _origin: Vector2,
        _size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        if let Ok(output) = event.output.downcast::<FT>() {
            (self.callback)(data, *output);
            return Some(());
        }
        None
    }
}

pub struct AsyncController<C, T, FT, B: Backend> {
    inner: AsyncControllerImpl<C, T, FT, B::Event, B::EventReaction>,
}

pub type AsyncControllerImpl<C, T, FT, E, R> = OrController<C, AsyncResolveController<T, FT>, E, R>;

impl<T, C, FT: 'static, B: Backend> AsyncController<C, T, FT, B>
where
    C: Controller<T>,
{
    pub fn new<CB: Fn(&mut T, FT) + 'static>(
        controller: C,
        callback: CB,
    ) -> AsyncController<C, T, FT, B> {
        let inner = controller.or(AsyncResolveController {
            callback: Box::new(callback),
        });
        AsyncController { inner }
    }
}

impl<C, T, FT, B: Backend> Controller<T> for AsyncController<C, T, FT, B>
where
    AsyncControllerImpl<C, T, FT, B::Event, B::EventReaction>: TypedController<T, B>,
{
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        TypedController::event(&mut self.inner, origin, size, data, event)
    }
}
