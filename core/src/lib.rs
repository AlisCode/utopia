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

    /// Shrink min and max constraints by size
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn shrink(&self, diff: impl Into<Size>) -> BoxConstraints {
        let diff = diff.into().expand();
        let min = Size::new(
            (self.min.width - diff.width).max(0.),
            (self.min.height - diff.height).max(0.),
        );
        let max = Size::new(
            (self.max.width - diff.width).max(0.),
            (self.max.height - diff.height).max(0.),
        );

        BoxConstraints { min, max }
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
