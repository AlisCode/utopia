use align::Align;
use padding::Padding;
use utopia_core::{widgets::TypedWidget, Backend};

use self::align::{HorizontalAlignment, VerticalAlignment};

pub mod align;
pub mod flex;
pub mod padding;

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
}

impl<T, B: Backend, W: TypedWidget<T, B> + Sized + 'static> LayoutExt<T, B> for W {}
