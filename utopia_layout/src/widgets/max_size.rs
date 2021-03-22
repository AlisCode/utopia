use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints,
};

use crate::SizeConstraint;

/// A Widget that forces its child to be at most as big as its constraint
pub struct MaxSize<T, B: Backend> {
    widget: WidgetPod<T, B>,
    constraint: SizeConstraint,
}

impl<T, B: Backend> MaxSize<T, B> {
    pub fn new<W: TypedWidget<T, B> + 'static>(widget: W, constraint: SizeConstraint) -> Self {
        MaxSize {
            widget: WidgetPod::new(widget),
            constraint,
        }
    }
}

impl<T, B: Backend> Widget<T> for MaxSize<T, B> {
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let width = self
            .constraint
            .width
            .solve(bc.max.width)
            .unwrap_or(bc.max.width);
        let height = self
            .constraint
            .height
            .solve(bc.max.height)
            .unwrap_or(bc.max.height);

        let child_bc = BoxConstraints {
            min: bc.min.clone(),
            max: Size {
                width: width.min(bc.max.width).max(bc.min.width),
                height: height.min(bc.max.height).max(bc.min.height),
            },
        };
        TypedWidget::<T, B>::layout(&mut self.widget, &child_bc, context, data)
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        TypedWidget::<T, B>::event(&mut self.widget, origin, size, data, event)
    }
}
