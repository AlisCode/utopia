use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints,
};

use crate::SizeConstraint;

/// A Widget that forces its child to be at least as big as its constraint
pub struct MinSize<T, B: Backend> {
    widget: WidgetPod<T, B>,
    constraint: SizeConstraint,
}

impl<T, B: Backend> MinSize<T, B> {
    pub fn new<W: TypedWidget<T, B> + 'static>(widget: W, constraint: SizeConstraint) -> Self {
        MinSize {
            widget: WidgetPod::new(widget),
            constraint,
        }
    }
}

impl<T, B: Backend> Widget<T> for MinSize<T, B> {
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = ();

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let width = self
            .constraint
            .width
            .solve(bc.max.width)
            .unwrap_or(bc.min.width);
        let height = self
            .constraint
            .height
            .solve(bc.max.height)
            .unwrap_or(bc.min.height);

        let child_bc = BoxConstraints {
            min: Size {
                width: width.max(bc.min.width).min(bc.max.width),
                height: height.max(bc.min.height).min(bc.max.height),
            },
            max: bc.max.clone(),
        };
        TypedWidget::<T, B>::layout(&mut self.widget, &child_bc, context, data)
    }
}
