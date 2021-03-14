use utopia_core::{controllers::click::Click, math::Size};

use nannou::prelude::*;
use utopia_nannou::{
    font::Font,
    interface::NannouInterface,
    widgets::{Align, Border, Controlled, Flex, LensWrap, Padding, Text},
};

fn main() {
    NannouInterface::run(model)
}

struct MyState {
    name: String,
    age: u32,
}

fn set_string(input: &mut u32) {
    *input += 1;
}

fn model(app: &App) -> NannouInterface<MyState> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let text = Text {
        font: Font::Default,
        font_size: 32,
        color: nannou::color::BLACK,
    };
    let text_other = Text {
        font: Font::Default,
        font_size: 16,
        color: nannou::color::RED,
    };
    let text_other = Padding::new(text_other).all(5);
    let text_other = Border::new(text_other).border_width(5);

    let lens_name = utopia_core::lens!(MyState, name);
    let lens_name_other = utopia_core::lens!(MyState, age);

    let click = Click::new(set_string);
    let controlled = Controlled::new(text_other, click);

    let mut flex = Flex::default();
    flex.add(LensWrap::new(Border::new(text).border_width(5), lens_name));
    flex.add(LensWrap::new(controlled, lens_name_other));

    let centered = Align::new(flex);

    let state = MyState {
        name: "Test Nannou".to_string(),
        age: 0,
    };

    NannouInterface::new(centered, state, size)
}
