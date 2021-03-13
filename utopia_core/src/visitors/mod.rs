use crate::{widgets::TypedWidget, Backend};

pub mod event;
pub mod layout;
pub mod paint;

pub trait Visitor<T, B: Backend> {
    type Output;
    fn visit<V: TypedWidget<T, B>>(&mut self, visitable: &mut V, backend: &B, data: &T);
    fn finish(self) -> Self::Output;
}

pub trait VisitorMut<T, B: Backend> {
    type Output;
    fn visit_mut<V: TypedWidget<T, B>>(&mut self, visitable: &mut V, backend: &B, data: &mut T);
    fn finish(self) -> Self::Output;
}
