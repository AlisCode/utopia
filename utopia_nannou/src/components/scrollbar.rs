use utopia_core::{component::Component, controllers::click::Click, widgets::pod::WidgetPod};
use utopia_layout::{
    spacer::{Axis, Spacer},
    SizeConstraint, ValueConstraint,
};
use utopia_scroll::widgets::scrollable::ScrollableState;

use crate::{
    widgets::{Flex, Label, WidgetExt},
    NannouBackend,
};

pub struct VerticalScrollbar {
    onclick_offset: f32,
}

impl Default for VerticalScrollbar {
    fn default() -> Self {
        VerticalScrollbar { onclick_offset: 5. }
    }
}

impl Component<ScrollableState, NannouBackend> for VerticalScrollbar {
    fn component(self) -> WidgetPod<ScrollableState, NannouBackend> {
        let VerticalScrollbar { onclick_offset } = self;

        let click_up = move |input: &mut ScrollableState| {
            input.offset_y -= onclick_offset;
        };

        let click_down = move |input: &mut ScrollableState| {
            input.offset_y += onclick_offset;
        };

        let widget = Flex::column()
            .add(
                Label::new("^")
                    .padding()
                    .all(5)
                    .border()
                    .max_size(SizeConstraint {
                        width: ValueConstraint::Pixels(20.),
                        height: ValueConstraint::Unconstrained,
                    })
                    .controlled(Click::new(click_up)),
            )
            .add_flex(
                Spacer::new(Axis::Vertical)
                    .min_size(SizeConstraint {
                        width: ValueConstraint::Pixels(20.),
                        height: ValueConstraint::Unconstrained,
                    })
                    .background()
                    .color(nannou::color::GRAY),
                1,
            )
            .add(
                Label::new("v")
                    .padding()
                    .all(5)
                    .border()
                    .max_size(SizeConstraint {
                        width: ValueConstraint::Pixels(20.),
                        height: ValueConstraint::Unconstrained,
                    })
                    .controlled(Click::new(click_down)),
            );

        WidgetPod::new(widget)
    }
}
