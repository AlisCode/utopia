use crate::{
    widgets::{pod::WidgetPod, TypedWidget},
    Backend,
};

pub trait BoxComponent<T, B: Backend>: TypedWidget<T, B> + Sized + 'static {}

pub trait Component<T, B: Backend> {
    fn component(self) -> WidgetPod<T, B>;
}
