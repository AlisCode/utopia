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

    let widget = Flex::column()
        .add(
            Text::new()
                .min_size(SizeConstraint {
                    width: ValueConstraint::percent(50.),
                    height: ValueConstraint::default(),
                })
                .border(),
        )
        .add(Text::new().border())
        .centered();
    let state = "Hello !";

    NannouInterface::new(widget, state, size)
}
