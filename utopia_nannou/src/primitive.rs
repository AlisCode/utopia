use crate::{font::Font, widgets::Color};
use nannou::{text::Scale, Draw};
use utopia_core::CommonPrimitive;
use utopia_decorations::primitives::quad::QuadPrimitive;
use utopia_text::primitives::text::TextPrimitive;

#[derive(Debug)]
pub enum NannouPrimitive {
    Common(CommonPrimitive<NannouPrimitive>),
    Text(TextPrimitive<Font, Color>),
    Quad(QuadPrimitive<Color>),
}

impl NannouPrimitive {
    pub fn draw(self, draw: &Draw, win_height: f32) {
        match self {
            NannouPrimitive::Common(common) => match common {
                CommonPrimitive::Group { children } => children
                    .into_iter()
                    .for_each(|prim| prim.draw(draw, win_height)),
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
                    .x_y(x, win_height - y);
            }
            NannouPrimitive::Quad(quad) => {
                let x = quad.origin.x + quad.size.width / 2.;
                let y = quad.origin.y + quad.size.height / 2.;
                draw.rect()
                    .x_y(x, win_height - y)
                    .w_h(
                        (quad.size.width - quad.border_width as f32 / 2.).ceil(),
                        (quad.size.height - quad.border_width as f32 / 2.).ceil(),
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
