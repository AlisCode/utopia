use scrollview::ScrollView;
use utopia_core::{widgets::TypedWidget, Backend};

pub mod scrollable;
pub mod scrollview;

pub trait ScrollExt<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {
    fn scroll(self) -> ScrollView<T, B> {
        ScrollView::new(self)
    }
}

impl<T, B: Backend, W: TypedWidget<T, B> + 'static> ScrollExt<T, B> for W {}
