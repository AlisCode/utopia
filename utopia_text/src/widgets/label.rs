use utopia_core::{
    math::{Size, Vector2},
    widgets::Widget,
    BoxConstraints,
};

use crate::{context::MeasureBrush, primitives::text::TextPrimitive};

pub struct Label<Font, Color> {
    content: &'static str,
    font_size: u16,
    font: Font,
    color: Color,
}

impl<Font: Default, Color: Default> Label<Font, Color> {
    pub fn new(content: &'static str) -> Self {
        Label {
            content,
            font_size: 16,
            font: Font::default(),
            color: Color::default(),
        }
    }
}

impl<T, Font: Clone, Color: Clone> Widget<T> for Label<Font, Color> {
    type Primitive = TextPrimitive<Font, Color>;
    type Context = MeasureBrush<Font>;
    type Event = ();
    type Reaction = ();

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, _data: &T) -> Size {
        let size = (context.measure)(self.content, self.font.clone(), self.font_size.clone());
        bc.constrain(size)
    }

    fn draw(&self, origin: Vector2, size: Size, _data: &T) -> Self::Primitive {
        TextPrimitive {
            content: self.content.to_string(),
            font: self.font.clone(),
            font_size: self.font_size,
            color: self.color.clone(),
            origin,
            size,
        }
    }
}
