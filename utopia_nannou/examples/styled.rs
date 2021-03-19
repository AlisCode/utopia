use utopia_core::{controllers::click::Click, math::Size};

use nannou::prelude::*;
use utopia_nannou::{
    interface::NannouInterface,
    widgets::{Color, Flex, LensExt, Styled, Text, WidgetExt},
};

fn main() {
    NannouInterface::run(model)
}

pub struct MyState {
    text_red: &'static str,
    text_blue: &'static str,
    text_green: &'static str,
    text: &'static str,
    pub text_color: Color,
}

fn on_click_red(input: &mut MyState) {
    input.text_color = nannou::color::RED;
}

fn on_click_green(input: &mut MyState) {
    input.text_color = nannou::color::GREEN;
}
fn on_click_blue(input: &mut MyState) {
    input.text_color = nannou::color::BLUE;
}

fn model(app: &App) -> NannouInterface<MyState> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let lens_red = utopia_core::lens!(MyState, text_red);
    let lens_green = utopia_core::lens!(MyState, text_green);
    let lens_blue = utopia_core::lens!(MyState, text_blue);

    let lens_text = utopia_core::lens!(MyState, text);
    let lens_color = utopia_core::lens!(MyState, text_color);
    let text_color = utopia_core::lens!(Text, color);

    let row = Flex::row()
        .add(
            Text::new()
                .lens(lens_red)
                .padding()
                .all(5)
                .border()
                .controlled(Click::new(on_click_red)),
        )
        .add(
            Text::new()
                .lens(lens_green)
                .padding()
                .all(5)
                .border()
                .controlled(Click::new(on_click_green)),
        )
        .add(
            Text::new()
                .lens(lens_blue)
                .padding()
                .all(5)
                .border()
                .controlled(Click::new(on_click_blue)),
        );

    let widget = Text::new().lens(lens_text);
    let widget = Styled::new(widget, lens_color, text_color);

    let col = Flex::column().add(row).add(widget);

    let state = MyState {
        text_red: "Red",
        text_green: "Green",
        text_blue: "Blue",
        text: "Hello !",
        text_color: nannou::color::PLUM,
    };

    NannouInterface::new(col.centered(), state, size)
}
