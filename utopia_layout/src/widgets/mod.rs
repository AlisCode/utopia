use align::Align;
use padding::Padding;
use utopia_core::{widgets::TypedWidget, Backend};

use crate::SizeConstraint;

use self::{
    align::{HorizontalAlignment, VerticalAlignment},
    max_size::MaxSize,
    min_size::MinSize,
};

pub mod align;
pub mod flex;
pub mod max_size;
pub mod min_size;
pub mod padding;
pub mod spacer;
pub mod stack;

pub trait LayoutExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn padding(self) -> Padding<T, B> {
        Padding::new(self)
    }

    fn align(self) -> Align<T, B> {
        Align::new(self)
    }

    fn centered(self) -> Align<T, B> {
        Align::new(self)
            .horizontal(HorizontalAlignment::Center)
            .vertical(VerticalAlignment::Center)
    }

    fn min_size(self, constraint: SizeConstraint) -> MinSize<T, B> {
        MinSize::new(self, constraint)
    }

    fn max_size(self, constraint: SizeConstraint) -> MaxSize<T, B> {
        MaxSize::new(self, constraint)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + Sized + 'static> LayoutExt<T, B> for W {}
