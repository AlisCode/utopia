use utopia_core::math::Size;

pub struct MeasureBrush<Font> {
    pub measure: Box<dyn Fn(&str, Font, u16) -> Size>,
}
