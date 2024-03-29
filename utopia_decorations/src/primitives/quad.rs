use utopia_core::math::{Size, Vector2};

#[derive(Debug)]
pub struct QuadPrimitive<Color> {
    pub color: Color,
    pub border_radius: u32,
    pub origin: Vector2,
    pub size: Size,
}
