use utopia_core::math::Size;

use nannou::prelude::*;
use utopia_layout::{SizeConstraint, ValueConstraint};
use utopia_nannou::{
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
        .max_size(SizeConstraint {
            height: ValueConstraint::Pixels(200.),
            width: Default::default(),
        })
        .min_size(SizeConstraint {
            width: ValueConstraint::Pixels(200.),
            height: Default::default(),
        })
        .scroll()
        .border()
        .centered();

    let state = "Hello !";

    NannouInterface::new(widget, state, size)
}
