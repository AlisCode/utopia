use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints, CommonPrimitive,
};

use super::scrollable::{Scrollable, ScrollableState};

pub struct ScrollView<T, B: Backend> {
    scroll: Scrollable<T, B>,
    scrollable_state: ScrollableState,
    vertical: Option<WidgetPod<ScrollableState, B>>,
    horizontal: Option<WidgetPod<ScrollableState, B>>,
}

impl<T, B: Backend> ScrollView<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(child: TW) -> Self {
        let scrollable_state = ScrollableState::default();
        let scroll = Scrollable::new(child);
        ScrollView {
            scroll,
            vertical: None,
            horizontal: None,
            scrollable_state: scrollable_state,
        }
    }

    pub fn horizontal(mut self, horizontal: WidgetPod<ScrollableState, B>) -> Self {
        self.horizontal = Some(horizontal);
        self
    }

    pub fn vertical(mut self, vertical: WidgetPod<ScrollableState, B>) -> Self {
        self.vertical = Some(vertical);
        self
    }
}

impl<T, B: Backend> Widget<T> for ScrollView<T, B>
where
    Scrollable<T, B>: TypedWidget<T, B>,
    B::Primitive: From<CommonPrimitive<B::Primitive>>,
    B::Event: Clone,
{
    type Primitive = CommonPrimitive<B::Primitive>;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let size = TypedWidget::<T, B>::layout(&mut self.scroll, bc, context, data);
        self.scroll.state = self.scrollable_state.clone();
        let scrollable_state = &self.scrollable_state;
        if let Some(vertical) = self.vertical.as_mut() {
            let bc = bc.loosen();
            let bar_size =
                TypedWidget::<ScrollableState, B>::layout(vertical, &bc, context, scrollable_state);
            vertical.set_origin(Vector2::new(size.width - bar_size.width, 0.));
        }
        if let Some(horizontal) = self.horizontal.as_mut() {
            let bc = bc.loosen();
            let bar_size = TypedWidget::<ScrollableState, B>::layout(
                horizontal,
                &bc,
                context,
                scrollable_state,
            );
            horizontal.set_origin(Vector2::new(0., size.height - bar_size.height));
        }

        Size {
            width: size.width,
            height: size.height,
        }
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        let scroll = TypedWidget::<T, B>::draw(&self.scroll, origin, size, data);
        let vertical = self
            .vertical
            .as_ref()
            .map(|vertical| {
                TypedWidget::<ScrollableState, B>::draw(
                    vertical,
                    origin,
                    size,
                    &self.scrollable_state,
                )
                .into()
            })
            .unwrap_or_else(|| CommonPrimitive::<B::Primitive>::None);
        let horizontal = self
            .horizontal
            .as_ref()
            .map(|horizontal| {
                TypedWidget::<ScrollableState, B>::draw(
                    horizontal,
                    origin,
                    size,
                    &self.scrollable_state,
                )
                .into()
            })
            .unwrap_or_else(|| CommonPrimitive::<B::Primitive>::None);
        CommonPrimitive::Group {
            children: vec![scroll, vertical.into(), horizontal.into()],
        }
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        if let Some(horizontal) = self.horizontal.as_mut() {
            if let Some(reaction) = TypedWidget::<ScrollableState, B>::event(
                horizontal,
                origin,
                size,
                &mut self.scrollable_state,
                event.clone(),
            ) {
                return Some(reaction);
            }
        }

        if let Some(vertical) = self.vertical.as_mut() {
            if let Some(reaction) = TypedWidget::<ScrollableState, B>::event(
                vertical,
                origin,
                size,
                &mut self.scrollable_state,
                event.clone(),
            ) {
                return Some(reaction);
            }
        }

        TypedWidget::<T, B>::event(&mut self.scroll, origin, size, data, event)
    }
}
