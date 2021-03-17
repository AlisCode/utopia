use utopia_core::math::{Size, Vector2};

#[derive(Debug)]
pub struct BorderPrimitive<Color> {
    pub border_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
    pub origin: Vector2,
    pub size: Size,
}
