use utopia_core::math::{Size, Vector2};

#[derive(Debug)]
pub struct TextPrimitive<Font, Color> {
    pub content: String,
    pub font: Font,
    pub font_size: u16,
    pub color: Color,
    pub origin: Vector2,
    pub size: Size,
}
