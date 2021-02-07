use super::Widget;

pub struct Border<Color> {
    pub border_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
}

pub struct QuadPrimitive<Color> {
    pub border_color: Color,
    pub border_radius: u32,
    pub border_width: u32,
}

impl<T, Color> Widget<T> for Border<Color> {
    type Primitive = QuadPrimitive<Color>;
    type Context = ();

    fn layout(
        &mut self,
        _bc: &crate::BoxConstraints,
        _context: &Self::Context,
        _data: &T,
    ) -> crate::math::Size {
        todo!()
    }

    fn draw(&self, _bounds: crate::math::Vector2, _data: &T) -> Self::Primitive {
        todo!()
    }
}
