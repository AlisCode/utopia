use utopia_core::{
    math::Size,
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints,
};

use crate::primitives::quad::QuadPrimitive;

pub struct Background<T, Color, B: Backend> {
    widget: WidgetPod<T, B>,
    color: Color,
}

impl<T, Color: Default, B: Backend> Background<T, Color, B> {
    pub fn new<W: TypedWidget<T, B> + 'static>(widget: W) -> Self {
        Background {
            widget: WidgetPod::new(widget),
            color: Color::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl<T, Color: Clone, B: Backend> Widget<T> for Background<T, Color, B> {
    type Primitive = (QuadPrimitive<Color>, B::Primitive);
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(
        &self,
        origin: utopia_core::math::Vector2,
        size: utopia_core::math::Size,
        data: &T,
    ) -> Self::Primitive {
        let child = TypedWidget::<T, B>::draw(&self.widget, origin, size, data);
        let new_size = Size {
            width: size.width,
            height: size.height,
        };
        let background = QuadPrimitive {
            origin,
            size: new_size,
            color: self.color.clone(),
            border_radius: 0,
        };
        (background, child)
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        TypedWidget::<T, B>::layout(&mut self.widget, bc, context, data)
    }
}
