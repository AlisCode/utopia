use crate::{
    math::{Size, Vector2},
    widgets::TypedWidget,
    Backend,
};

pub struct PaintStep<P> {
    pub size: Size,
    primitive: std::marker::PhantomData<P>,
}

impl<P> Default for PaintStep<P> {
    fn default() -> Self {
        PaintStep {
            size: Size::default(),
            primitive: std::marker::PhantomData,
        }
    }
}

impl<P> PaintStep<P> {
    pub fn apply<T, B, TW: TypedWidget<T, B>>(&self, widget: &TW, data: &T) -> P
    where
        B: Backend<Primitive = P>,
    {
        <TW as TypedWidget<T, B>>::draw(widget, Vector2::ZERO, self.size, data)
    }
}
