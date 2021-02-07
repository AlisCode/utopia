use crate::math::Vector2;

use super::Widget;

pub struct WidgetPod<T, P, C> {
    widget: Box<dyn Widget<T, Primitive = P, Context = C>>,
    state: WidgetState,
}

impl<T, P, C> WidgetPod<T, P, C> {
    pub fn new<W: Widget<T, Primitive = P, Context = C> + 'static>(widget: W) -> Self {
        WidgetPod {
            widget: Box::new(widget),
            state: WidgetState::default(),
        }
    }

    pub fn set_origin(&mut self, origin: Vector2) {
        self.state.origin = origin
    }
}

#[derive(Debug, Default)]
pub struct WidgetState {
    origin: Vector2,
}

impl<T, P, C> Widget<T> for WidgetPod<T, P, C> {
    type Primitive = P;
    type Context = C;

    fn layout(
        &mut self,
        bc: &crate::BoxConstraints,
        context: &Self::Context,
        data: &T,
    ) -> crate::math::Size {
        self.widget.layout(bc, context, data)
    }

    fn draw(&self, origin: Vector2, data: &T) -> Self::Primitive {
        // Maybe change the position ?
        self.widget.draw(origin + self.state.origin, data)
    }
}
