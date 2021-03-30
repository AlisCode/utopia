use crate::{font::Font, widgets::Color};
use nannou::{geom::rect::Rect, prelude::Vector3, text::Scale, wgpu::Texture, Draw};
use utopia_core::CommonPrimitive;
use utopia_decorations::{
    primitives::{border::BorderPrimitive, quad::QuadPrimitive},
    widgets::scale::ScaledPrimitive,
};
use utopia_image::primitive::ImagePrimitive;
use utopia_scroll::primitive::ClipPrimitive;
use utopia_text::primitives::text::TextPrimitive;

#[derive(Debug)]
pub enum NannouPrimitive {
    Common(CommonPrimitive<NannouPrimitive>),
    Text(TextPrimitive<Font, Color>),
    Quad(QuadPrimitive<Color>),
    Border(BorderPrimitive<Color>),
    Image(ImagePrimitive<Texture>),
    Clip(ClipPrimitive<NannouPrimitive>),
    Scaled(ScaledPrimitive<NannouPrimitive>),
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
                    .w_h(quad.size.width.ceil(), quad.size.height.ceil())
                    .color(quad.color);
            }
            NannouPrimitive::Border(border) => {
                let x = border.origin.x + border.size.width / 2.;
                let y = border.origin.y + border.size.height / 2.;
                draw.rect()
                    .x_y(x, win_height - y)
                    .w_h(
                        (border.size.width - border.border_width as f32 / 2.).ceil(),
                        (border.size.height - border.border_width as f32 / 2.).ceil(),
                    )
                    .no_fill()
                    .stroke_weight(border.border_width as f32)
                    .stroke(border.border_color);
            }
            NannouPrimitive::Image(image) => {
                let x = image.position.x + image.size.width / 2.;
                let y = image.position.y + image.size.height / 2.;
                draw.texture(&image.src)
                    .x_y(x, y)
                    .w_h(image.size.width, image.size.height);
            }
            NannouPrimitive::Clip(clip) => {
                let x = clip.bounds.width / 2.;
                let x = nannou::geom::Range::new(-x, x);
                let y = clip.bounds.height / 2.;
                let y = nannou::geom::Range::new(-y, y);

                let scissor = draw
                    .scissor(Rect { x, y })
                    .x_y(-clip.offset.x, clip.offset.y);
                clip.primitive.draw(&scissor, win_height);
            }
            NannouPrimitive::Scaled(scaled) => {
                // FIXME: Not scaled with proper origin ?
                let scaled_draw = draw.scale_axes(Vector3::new(scaled.scale_x, scaled.scale_y, 1.));
                scaled.primitive.draw(&scaled_draw, win_height);
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

impl From<BorderPrimitive<Color>> for NannouPrimitive {
    fn from(input: BorderPrimitive<Color>) -> Self {
        NannouPrimitive::Border(input)
    }
}

impl From<ImagePrimitive<Texture>> for NannouPrimitive {
    fn from(input: ImagePrimitive<Texture>) -> Self {
        NannouPrimitive::Image(input)
    }
}

impl From<ClipPrimitive<NannouPrimitive>> for NannouPrimitive {
    fn from(input: ClipPrimitive<NannouPrimitive>) -> Self {
        NannouPrimitive::Clip(input)
    }
}

impl From<ScaledPrimitive<NannouPrimitive>> for NannouPrimitive {
    fn from(input: ScaledPrimitive<NannouPrimitive>) -> Self {
        NannouPrimitive::Scaled(input)
    }
}

impl From<()> for NannouPrimitive {
    fn from(_input: ()) -> Self {
        NannouPrimitive::Common(CommonPrimitive::None)
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
