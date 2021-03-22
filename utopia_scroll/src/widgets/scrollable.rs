use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints, CommonPrimitive,
};

use crate::primitive::ClipPrimitive;

#[derive(Debug, Default, Clone)]
pub struct ScrollableState {
    pub offset_x: f32,
    pub offset_y: f32,
    scroll_x: ScrollType,
    scroll_y: ScrollType,
    /// Read-only (changing this won't affect the size of child_size)
    child_size: Size,
}

pub struct Scrollable<T, B: Backend> {
    child: WidgetPod<T, B>,
    pub scrollbar_vertical: Option<WidgetPod<ScrollableState, B>>,
    pub scrollbar_horizontal: Option<WidgetPod<ScrollableState, B>>,
    pub state: ScrollableState,
}

impl<T, B: Backend> Scrollable<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(child: TW) -> Self {
        Scrollable {
            child: WidgetPod::new(child),
            scrollbar_vertical: None,
            scrollbar_horizontal: None,
            state: ScrollableState::default(),
        }
    }
}

impl<T, B: Backend> Widget<T> for Scrollable<T, B> {
    type Primitive = (ClipPrimitive<B::Primitive>, CommonPrimitive<B::Primitive>);
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        let child_primitive = TypedWidget::<T, B>::draw(&self.child, origin, size, data);
        (
            ClipPrimitive {
                origin,
                offset: Vector2::new(self.state.offset_x, self.state.offset_y),
                bounds: size,
                primitive: Box::new(child_primitive),
            },
            CommonPrimitive::None,
        )
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let child_size = TypedWidget::<T, B>::layout(&mut self.child, bc, context, data);
        self.state.child_size = child_size.clone();
        child_size
    }
}

#[derive(Debug, Clone)]
pub enum ScrollType {
    Hidden,
    Scroll,
}

impl Default for ScrollType {
    fn default() -> Self {
        ScrollType::Scroll
    }
}
