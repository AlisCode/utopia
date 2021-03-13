use border::Border;
use utopia_core::{widgets::TypedWidget, Backend};

pub mod border;

pub trait DecorationsExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn bordered<Color: Default>(self) -> Border<T, Color, B> {
        Border::new(self)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + 'static> DecorationsExt<T, B> for W {}
