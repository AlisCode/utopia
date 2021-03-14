use crate::{math::Size, widgets::TypedWidget, Backend, BoxConstraints};

pub struct LayoutStep {
    pub box_constraints: BoxConstraints,
}

impl Default for LayoutStep {
    fn default() -> Self {
        LayoutStep {
            box_constraints: BoxConstraints {
                min: Size::default(),
                max: Size::default(),
            },
        }
    }
}

impl LayoutStep {
    pub fn apply<T, B: Backend, TW: TypedWidget<T, B>>(
        &mut self,
        widget: &mut TW,
        backend: &B,
        data: &T,
    ) -> Size {
        <TW as TypedWidget<T, B>>::layout(widget, &self.box_constraints, backend, data)
    }
}
