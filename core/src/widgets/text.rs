use crate::{
    math::{Size, Vector2},
    widgets::Widget,
};

#[derive(Debug)]
pub struct Text<Font, Color> {
    pub font: Font,
    pub font_size: u16,
    pub color: Color,
}

#[derive(Debug)]
pub struct TextPrimitive<Font, Color> {
    pub content: String,
    pub font: Font,
    pub font_size: u16,
    pub color: Color,
    pub origin: Vector2,
}

pub struct MeasureBrush<Font> {
    pub measure: Box<dyn Fn(&str, Font, u16) -> Size>,
}

impl<Font: Copy, Color: Copy> Widget<String> for Text<Font, Color> {
    type Primitive = TextPrimitive<Font, Color>;
    type Context = MeasureBrush<Font>;

    fn layout(
        &mut self,
        bc: &crate::BoxConstraints,
        context: &Self::Context,
        data: &String,
    ) -> crate::Size {
        let size = (context.measure)(data.as_str(), self.font, self.font_size);
        bc.constrain(size)
    }

    fn draw(&self, origin: Vector2, data: &String) -> Self::Primitive {
        TextPrimitive {
            content: data.clone(),
            font: self.font,
            font_size: self.font_size,
            color: self.color,
            origin,
        }
    }
}
