use std::fmt::Display;
use utopia_core::{
    math::{Size, Vector2},
    widgets::Widget,
    BoxConstraints,
};

use crate::context::MeasureBrush;
use crate::primitives::text::TextPrimitive;

#[derive(Debug)]
pub struct Text<Font, Color> {
    pub font: Font,
    pub font_size: u16,
    pub color: Color,
}

impl<Font: Default, Color: Default> Default for Text<Font, Color> {
    fn default() -> Self {
        Text {
            font: Font::default(),
            font_size: 16,
            color: Color::default(),
        }
    }
}

impl<Font: Default, Color: Default> Text<Font, Color> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Display, Font: Clone, Color: Clone> Widget<T> for Text<Font, Color> {
    type Primitive = TextPrimitive<Font, Color>;
    type Context = MeasureBrush<Font>;
    type Event = ();
    type Reaction = ();

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let data = data.to_string();
        let size = (context.measure)(data.as_str(), self.font.clone(), self.font_size.clone());
        bc.constrain(size)
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TextPrimitive {
            content: data.to_string(),
            font: self.font.clone(),
            font_size: self.font_size,
            color: self.color.clone(),
            origin,
            size,
        }
    }
}
