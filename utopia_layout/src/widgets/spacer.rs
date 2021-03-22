use utopia_core::{math::Size, widgets::Widget};

pub struct Spacer {
    axis: Axis,
}

impl Spacer {
    pub fn new(axis: Axis) -> Self {
        Spacer { axis }
    }
}

pub enum Axis {
    Vertical,
    Horizontal,
}

impl<T> Widget<T> for Spacer {
    type Primitive = ();
    type Context = ();
    type Event = ();
    type Reaction = ();

    fn draw(
        &self,
        _origin: utopia_core::math::Vector2,
        _size: utopia_core::math::Size,
        _data: &T,
    ) -> Self::Primitive {
    }

    fn layout(
        &mut self,
        bc: &utopia_core::BoxConstraints,
        _context: &Self::Context,
        _data: &T,
    ) -> utopia_core::math::Size {
        Size {
            width: match self.axis {
                Axis::Horizontal if bc.max.width.is_finite() => bc.max.width,
                _ => bc.min.width,
            },
            height: match self.axis {
                Axis::Vertical if bc.max.height.is_finite() => bc.max.height,
                _ => bc.min.height,
            },
        }
    }
}
