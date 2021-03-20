use utopia_core::math::{Size, Vector2};

#[derive(Debug)]
pub struct ClipPrimitive<P> {
    pub bounds: Size,
    pub origin: Vector2,
    pub offset: Vector2,
    pub primitive: Box<P>,
}
