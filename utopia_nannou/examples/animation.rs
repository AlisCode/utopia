use utopia_core::{lens, math::Size};

use nannou::prelude::*;
use utopia_nannou::{
    interface::NannouInterface,
    widgets::{Scale, Text, WidgetExt},
};

fn main() {
    NannouInterface::run(model)
}

fn model(app: &App) -> NannouInterface<&'static str> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let lens_x = lens!(Scale::<&'static str>, scale_x);
    let lens_y = lens!(Scale::<&'static str>, scale_y);

    let widget = Text::new()
        .border()
        .scaled()
        .boxed()
        .animate(lens_x, 1.5)
        .animate(lens_y, 1.5);

    let state = "Hello !";

    NannouInterface::new(widget.centered(), state, size)
}
