use crate::{
    math::{Size, Vector2},
    Backend, BoxConstraints,
};

use super::{pod::WidgetPod, TypedWidget, Widget};

pub struct Padding<T, B: Backend> {
    widget: WidgetPod<T, B>,
    padding_left: u32,
    padding_top: u32,
    padding_right: u32,
    padding_bottom: u32,
}

impl<T, B: Backend> Padding<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(widget: TW) -> Self {
        Padding {
            widget: WidgetPod::new(widget),
            padding_left: 0,
            padding_right: 0,
            padding_top: 0,
            padding_bottom: 0,
        }
    }

    pub fn top(mut self, padding_top: u32) -> Self {
        self.padding_top = padding_top;
        self
    }

    pub fn bottom(mut self, padding_bottom: u32) -> Self {
        self.padding_bottom = padding_bottom;
        self
    }

    pub fn left(mut self, padding_left: u32) -> Self {
        self.padding_left = padding_left;
        self
    }

    pub fn right(mut self, padding_right: u32) -> Self {
        self.padding_right = padding_right;
        self
    }

    pub fn all(mut self, padding: u32) -> Self {
        self.padding_bottom = padding;
        self.padding_left = padding;
        self.padding_right = padding;
        self.padding_top = padding;
        self
    }
}

impl<T, B: Backend> Widget<T> for Padding<T, B> {
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let width = (self.padding_left + self.padding_right) as f32;
        let height = (self.padding_top + self.padding_bottom) as f32;
        let size = Size { width, height };
        let child_bc = bc.shrink(size);
        self.widget.set_origin(Vector2 {
            x: self.padding_left as f32,
            y: self.padding_top as f32,
        });
        let child_size = TypedWidget::<T, B>::layout(&mut self.widget, &child_bc, context, data);
        Size {
            width: width + child_size.width,
            height: height + child_size.height,
        }
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
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
