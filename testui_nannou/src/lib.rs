use core::{
    contexts::ContextProvider,
    math::Size,
    widgets::{
        border::{Border as BorderWidget, QuadPrimitive},
        flex::Flex as FlexWidget,
        pod::WidgetPod,
        text::Text as TextWidget,
        text::{MeasureBrush, TextPrimitive},
    },
    Backend, CommonPrimitive,
};

use nannou::{
    geom::rect::Rect,
    text::{Builder as TextBuilder, Scale},
    Draw,
};

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

pub type Color = nannou::color::Srgb<u8>;
pub type NannouWidgetPod<T> = WidgetPod<T, NannouBackend>;
pub type Flex<T> = FlexWidget<T, NannouBackend>;
pub type Text = TextWidget<Font, Color>;
pub type Border<T> = BorderWidget<T, Color, NannouBackend>;

#[derive(Debug, Clone)]
pub enum Font {
    Default,
    Font(nannou::text::Font),
}

impl Font {
    fn resolve(&self) -> nannou::text::Font {
        match self {
            Font::Default => nannou::text::font::default_notosans(),
            Font::Font(font) => font.clone(),
        }
    }
}

#[derive(Debug)]
pub enum NannouPrimitive {
    Common(CommonPrimitive<NannouPrimitive>),
    Text(TextPrimitive<Font, Color>),
    Quad(QuadPrimitive<Color>),
}

impl NannouPrimitive {
    pub fn draw(self, draw: &Draw) {
        match self {
            NannouPrimitive::Common(common) => match common {
                CommonPrimitive::Group { children } => {
                    children.into_iter().for_each(|prim| prim.draw(draw))
                }
                _ => {}
            },
            NannouPrimitive::Text(text) => {
                if text.content == "" {
                    return;
                }
                let font = text.font.resolve();
                let v_metrics = font.v_metrics(Scale::uniform(text.font_size as f32));
                let x = text.origin.x + text.size.width / 2.;
                let y = text.origin.y + text.size.height / 2. + v_metrics.descent;
                draw.text(&text.content)
                    .color(text.color)
                    .font_size(text.font_size as u32)
                    .x_y(x, -y);
            }
            NannouPrimitive::Quad(quad) => {
                draw.rect()
                    .x_y(
                        quad.origin.x + quad.size.width / 2.,
                        quad.origin.y - quad.size.height / 2.,
                    )
                    .w_h(
                        quad.size.width - quad.border_width as f32,
                        quad.size.height - quad.border_width as f32,
                    )
                    .no_fill()
                    .stroke_weight(quad.border_width as f32)
                    .stroke(quad.border_color);
            }
        }
    }
}

impl From<CommonPrimitive<NannouPrimitive>> for NannouPrimitive {
    fn from(input: CommonPrimitive<NannouPrimitive>) -> Self {
        NannouPrimitive::Common(input)
    }
}

impl From<TextPrimitive<Font, Color>> for NannouPrimitive {
    fn from(input: TextPrimitive<Font, Color>) -> Self {
        NannouPrimitive::Text(input)
    }
}

impl From<QuadPrimitive<Color>> for NannouPrimitive {
    fn from(input: QuadPrimitive<Color>) -> Self {
        NannouPrimitive::Quad(input)
    }
}

impl<A, B> From<(A, B)> for NannouPrimitive
where
    NannouPrimitive: From<A>,
    NannouPrimitive: From<B>,
{
    fn from((a, b): (A, B)) -> NannouPrimitive {
        NannouPrimitive::Common(CommonPrimitive::Group {
            children: vec![a.into(), b.into()],
        })
    }
}

impl Backend for NannouBackend {
    type Primitive = NannouPrimitive;
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
