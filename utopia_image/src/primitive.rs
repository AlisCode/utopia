use utopia_core::math::{Size, Vector2};

#[derive(Debug)]
pub struct ImagePrimitive<Img> {
    pub position: Vector2,
    pub size: Size,
    pub src: Img,
}
