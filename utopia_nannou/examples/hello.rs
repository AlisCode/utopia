use utopia_core::math::Size;

use nannou::prelude::*;
use utopia_nannou::{
    interface::NannouInterface,
    widgets::{Text, WidgetExt},
};

fn main() {
    NannouInterface::run(model)
}

fn model(app: &App) -> NannouInterface<&'static str> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let widget = Text::new().centered();
    let state = "Hello !";

    NannouInterface::new(widget, state, size)
}
