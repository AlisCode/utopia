use crate::font::Font;
use nannou::{geom::rect::Rect, text::Builder as TextBuilder};
use utopia_core::{contexts::ContextProvider, math::Size, Backend};
use utopia_text::context::MeasureBrush;

pub mod event;
pub mod font;
pub mod interface;
pub mod primitive;
pub mod widgets;

pub struct NannouBackend {
    measure_brush: MeasureBrush<Font>,
}

impl Default for NannouBackend {
    fn default() -> Self {
        let measure_brush = MeasureBrush {
            measure: Box::new(measure),
        };
        NannouBackend { measure_brush }
    }
}

impl Backend for NannouBackend {
    type Primitive = primitive::NannouPrimitive;
    type Event = event::NannouEvent;
    type EventReaction = ();
}

fn measure(contents: &str, font: Font, size: u16) -> Size {
    let text = TextBuilder::from(contents).font_size(size as nannou::text::FontSize);

    let text = match font {
        Font::Font(font) => text.font(font),
        _ => text,
    }
    .build(Rect::from_w_h(300., 300.));
    let rect = text.bounding_rect();
    Size {
        width: rect.w(),
        height: rect.h(),
    }
}

impl ContextProvider<MeasureBrush<Font>> for NannouBackend {
    fn provide(&self) -> &MeasureBrush<Font> {
        &self.measure_brush
    }
}
