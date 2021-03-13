use crate::{math::Size, widgets::TypedWidget, Backend, BoxConstraints};

use super::Visitor;

pub struct LayoutVisitor {
    pub box_constraints: BoxConstraints,
    pub final_size: Option<Size>,
}

impl<'a, B: Backend, T> Visitor<T, B> for LayoutVisitor {
    type Output = Size;

    fn visit<TW: TypedWidget<T, B>>(&mut self, widget: &mut TW, backend: &B, data: &T) {
        let size = <TW as TypedWidget<T, B>>::layout(widget, &self.box_constraints, backend, data);
        self.final_size = Some(size);
    }

    fn finish(self) -> Self::Output {
        self.final_size.unwrap()
    }
}
