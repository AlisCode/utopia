use utopia_core::math::Size;

use nannou::{prelude::*, wgpu::Texture};
use utopia_nannou::{
    interface::NannouInterface,
    widgets::{Flex, Image, LensExt, WidgetExt},
};
use utopia_text::widgets::text::Text;

fn main() {
    NannouInterface::run(model)
}

struct MyState {
    text: &'static str,
    texture: Texture,
}

fn model(app: &App) -> NannouInterface<MyState> {
    let rect = app.window_rect();
    let size = Size::new(rect.w(), rect.h());

    let lens_img = utopia_core::lens!(MyState, texture);
    let lens_text = utopia_core::lens!(MyState, text);

    let assets = app.assets_path().unwrap();
    let widget = Flex::column()
        .add(Image::new().lens(lens_img).border())
        .add(Text::new().lens(lens_text).centered().border())
        .border()
        .centered();

    let state = MyState {
        texture: Texture::from_path(app, assets.join("texture.png"))
            .expect("Failed to load texture"),
        text: "Hello world",
    };

    NannouInterface::new(widget, state, size)
}
