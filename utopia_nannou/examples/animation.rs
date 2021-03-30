use utopia_core::{lens, math::Size};

use nannou::prelude::*;
use utopia_nannou::{
    interface::NannouInterface,
    widgets::{Animated, Scale, Text, WidgetExt},
};

fn main() {
    NannouInterface::run(model)
}

fn model(app: &App) -> NannouInterface<&'static str> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let lens_x = lens!(Scale::<&'static str>, scale_x);
    let lens_y = lens!(Scale::<&'static str>, scale_y);

    let widget = Text::new().border().scaled();
    let widget_a =
        Animated::<&'static str, _, _, _, _, _, _>::animate_to(Box::new(widget), lens_x, 1.5);
    let widget_b = Animated::animate_to(widget_a, lens_y, 1.5);

    let state = "Hello !";

    NannouInterface::new(widget_b.centered(), state, size)
}
