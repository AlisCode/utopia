use crate::{
    math::{Size, Vector2},
    widgets::TypedWidget,
    Backend,
};

use super::Visitor;

pub struct PaintVisitor<P> {
    pub size: Size,
    pub primitive: P,
}

impl<'a, B: Backend, T> Visitor<T, B> for PaintVisitor<B::Primitive> {
    type Output = B::Primitive;

    fn visit<TW: TypedWidget<T, B>>(&mut self, widget: &mut TW, _backend: &B, data: &T) {
        self.primitive = <TW as TypedWidget<T, B>>::draw(widget, Vector2::ZERO, self.size, data);
    }

    fn finish(self) -> Self::Output {
        self.primitive
    }
}
