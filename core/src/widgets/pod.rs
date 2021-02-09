use crate::{
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

use super::{TypedWidget, Widget};

pub struct WidgetPod<T, B: Backend> {
    widget: Box<dyn TypedWidget<T, B>>,
    state: WidgetState,
}

impl<T, B: Backend> WidgetPod<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(typed_widget: TW) -> Self {
        WidgetPod {
            widget: Box::new(typed_widget),
            state: WidgetState::default(),
        }
    }

    pub fn set_origin(&mut self, origin: Vector2) {
        self.state.origin = origin
    }
}

impl<T, B: Backend> Widget<T> for WidgetPod<T, B> {
    type Primitive = B::Primitive;
    type Context = B;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        TypedWidget::<T, B>::layout(self.widget.as_mut(), bc, context, data)
    }

    fn draw(&self, origin: Vector2, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(self.widget.as_ref(), origin + self.state.origin, data)
    }
}

#[derive(Debug, Default)]
pub struct WidgetState {
    origin: Vector2,
}
