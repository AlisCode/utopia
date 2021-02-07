use crate::{widgets::TypedWidget, Backend};

pub mod layout;
pub mod paint;

pub trait Visitor<T, B: Backend> {
    type Output;
    fn visit<V: TypedWidget<T, B>>(&mut self, visitable: &mut V, backend: &B, data: &T);
    fn finish(self) -> Self::Output;
}
