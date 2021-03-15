use utopia_core::math::Size;

pub struct ImageContext<Img> {
    pub measure: Box<dyn Fn(&Img) -> Size>,
}
