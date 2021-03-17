use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints,
};

use crate::primitives::border::BorderPrimitive;

pub struct Border<T, Color, B: Backend> {
    pub border_color: Color,
    pub background_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
    pub widget: WidgetPod<T, B>,
}

impl<T, Color: Default, B: Backend> Border<T, Color, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(widget: TW) -> Self {
        Border {
            border_color: Color::default(),
            background_color: Color::default(),
            border_radius: 0,
            border_width: 1,
            widget: WidgetPod::new(widget),
        }
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = width;
        self
    }

    pub fn border_radius(mut self, radius: u32) -> Self {
        self.border_radius = radius;
        self
    }
}

impl<T, Color: Clone, B: Backend> Widget<T> for Border<T, Color, B> {
    type Primitive = (BorderPrimitive<Color>, B::Primitive);
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let border_width = self.border_width as f32;
        let double_border_width = border_width * 2.;
        let child_bc = bc.shrink((double_border_width, double_border_width));
        let child_size = TypedWidget::<T, B>::layout(&mut self.widget, &child_bc, context, data);
        self.widget.set_origin(Vector2 {
            x: border_width,
            y: border_width,
        });

        Size {
            width: child_size.width + double_border_width,
            height: child_size.height + double_border_width,
        }
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        let border = BorderPrimitive {
            border_color: self.border_color.clone(),
            border_radius: self.border_radius,
            border_width: self.border_width,
            origin,
            size,
        };
        let inner = TypedWidget::<T, B>::draw(&self.widget, origin, size, data);
        (border, inner)
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
