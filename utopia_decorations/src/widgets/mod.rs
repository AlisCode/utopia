use border::Border;
use utopia_core::{widgets::TypedWidget, Backend};

use self::background::Background;

pub mod background;
pub mod border;

pub trait DecorationsExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn border<Color: Default>(self) -> Border<T, Color, B> {
        Border::new(self)
    }

    fn background<Color: Default>(self) -> Background<T, Color, B> {
        Background::new(self)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + 'static> DecorationsExt<T, B> for W {}
