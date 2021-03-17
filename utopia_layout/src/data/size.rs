#[derive(Debug, Clone, PartialEq)]
pub enum ValueConstraint {
    Pixels(f32),
    Percent(f32),
    Unconstrained,
}

impl ValueConstraint {
    pub fn solve(&self, parent_size: f32) -> Option<f32> {
        match self {
            ValueConstraint::Percent(percent) => match parent_size.is_finite() {
                true => Some(parent_size * percent),
                false => None,
            },
            ValueConstraint::Pixels(pixels) => Some(*pixels),
            ValueConstraint::Unconstrained => None,
        }
    }

    pub fn pixels(pixels: f32) -> Self {
        ValueConstraint::Pixels(pixels)
    }

    pub fn percent(percent: f32) -> Self {
        ValueConstraint::Percent(percent / 100.)
    }

    pub fn unconstrained() -> Self {
        ValueConstraint::Unconstrained
    }
}

impl Default for ValueConstraint {
    fn default() -> Self {
        ValueConstraint::Unconstrained
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SizeConstraint {
    pub width: ValueConstraint,
    pub height: ValueConstraint,
}
