use crate::{
    math::{Size, Vector2},
    Backend,
};

use super::{pod::WidgetPod, TypedWidget, Widget};

pub struct Border<T, Color, B: Backend> {
    pub border_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
    pub widget: WidgetPod<T, B>,
}

impl<T, Color: Default, B: Backend> Border<T, Color, B> {
    pub fn new<TW: TypedWidget<T, B, Primitive = B::Primitive, Context = B> + 'static>(
        widget: TW,
    ) -> Self {
        Border {
            border_color: Color::default(),
            border_radius: 0,
            border_width: 1,
            widget: WidgetPod::new(widget),
        }
    }
}

#[derive(Debug)]
pub struct QuadPrimitive<Color> {
    pub border_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
    pub origin: Vector2,
}

impl<T, Color: Clone, B: Backend> Widget<T> for Border<T, Color, B> {
    type Primitive = (QuadPrimitive<Color>, B::Primitive);
    type Context = B;

    fn layout(
        &mut self,
        bc: &crate::BoxConstraints,
        context: &Self::Context,
        data: &T,
    ) -> crate::math::Size {
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

    fn draw(&self, origin: crate::math::Vector2, data: &T) -> Self::Primitive {
        let quad = QuadPrimitive {
            border_color: self.border_color.clone(),
            border_radius: self.border_radius,
            border_width: self.border_width,
            origin,
        };
        let inner = TypedWidget::<T, B>::draw(&self.widget, origin, data);
        (quad, inner)
    }
}
