use utopia_core::{component::Component, math::Size};

use nannou::prelude::*;
use utopia_layout::{SizeConstraint, ValueConstraint};
use utopia_nannou::{
    components::scrollbar::VerticalScrollbar,
    interface::NannouInterface,
    widgets::{Flex, Text, WidgetExt},
};

fn main() {
    NannouInterface::run(model)
}

fn model(app: &App) -> NannouInterface<&'static str> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let widget = (0..30)
        .fold(Flex::column(), |flex, _| flex.add(Text::new()))
        .scroll()
        .vertical(VerticalScrollbar::default().component())
        .border()
        .max_size(SizeConstraint {
            height: ValueConstraint::Pixels(200.),
            width: ValueConstraint::Pixels(300.),
        })
        .min_size(SizeConstraint {
            width: ValueConstraint::Pixels(200.),
            height: Default::default(),
        })
        .centered();

    let state = "Hello !";

    NannouInterface::new(widget, state, size)
}
