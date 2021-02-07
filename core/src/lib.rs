use std::sync::Arc;

use math::{Size, Vector2};

pub mod contexts;
pub mod math;
pub mod visitors;
pub mod widgets;

#[derive(Debug)]
pub struct BoxConstraints {
    pub min: Size,
    pub max: Size,
}

impl BoxConstraints {
    /// Create a "loose" version of the constraints.
    ///
    /// Make a version with zero minimum size, but the same maximum size.
    pub fn loosen(&self) -> BoxConstraints {
        BoxConstraints {
            min: Size::ZERO,
            max: self.max,
        }
    }

    /// Clamp a given size so that it fits within the constraints.
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn constrain(&self, size: impl Into<Size>) -> Size {
        size.into().expand().clamp(self.min, self.max)
    }
}

#[derive(Debug)]
pub enum CommonPrimitive<P> {
    None,
    Group {
        children: Vec<P>,
    },
    Cached {
        cache: Arc<P>,
    },
    Clip {
        bounds: Size,
        offset: Vector2,
        content: Box<P>,
    },
}

impl<P> From<P> for CommonPrimitive<P> {
    fn from(input: P) -> CommonPrimitive<P> {
        CommonPrimitive::Group {
            children: vec![input],
        }
    }
}

pub trait Backend {
    type Primitive: From<CommonPrimitive<Self::Primitive>>;
}
