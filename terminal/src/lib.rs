use core::{
    contexts::ContextProvider,
    math::Size,
    widgets::{
        border::QuadPrimitive,
        text::{MeasureBrush, TextPrimitive},
    },
    Backend, CommonPrimitive,
};
use std::io::{Stdout, Write};

use terminal::{Action, Clear, Terminal};
pub struct TerminalBackend {
    measure_brush: MeasureBrush<()>,
    pub terminal: Terminal<Stdout>,
}

pub type Align<T> = core::widgets::align::Align<T, TerminalBackend>;
pub type Color = ();
pub type Flex<T> = core::widgets::flex::Flex<T, TerminalBackend>;
pub type Text = core::widgets::text::Text<(), Color>;
pub type Border<T> = core::widgets::border::Border<T, Color, TerminalBackend>;

fn display_blank(
    terminal: &mut Terminal<Stdout>,
    origin_x: u16,
    y: u16,
    blank_line: &str,
) -> terminal::error::Result<()> {
    terminal.batch(Action::MoveCursorTo(origin_x, y))?;
    terminal.write(blank_line.as_bytes())?;
    Ok(())
}

impl TerminalBackend {
    pub fn draw_primitive(&mut self, primitive: TerminalPrimitive) -> terminal::error::Result<()> {
        match primitive {
            TerminalPrimitive::Common(common) => match common {
                CommonPrimitive::Group { children } => {
                    children
                        .into_iter()
                        .map(|primitive| self.draw_primitive(primitive))
                        .collect::<Result<_, _>>()?;
                }
                _ => (),
            },
            TerminalPrimitive::Text(text) => {
                self.terminal.batch(Action::MoveCursorTo(
                    text.origin.x as u16,
                    text.origin.y as u16,
                ))?;
                self.terminal.write(text.content.as_bytes())?;
            }
            TerminalPrimitive::Quad(quad) => {
                let width = quad.size.width as usize;
                let height = quad.size.height as usize;
                if width < 2 || height < 2 {
                    return Ok(());
                }
                let full_line: String = Some('+')
                    .into_iter()
                    .chain((0..width - 2).map(|_| '-'))
                    .chain(Some('+').into_iter())
                    .collect();
                let blank_line: String = Some('|')
                    .into_iter()
                    .chain((0..width - 2).map(|_| ' '))
                    .chain(Some('|').into_iter())
                    .collect();
                self.terminal.batch(Action::MoveCursorTo(
                    quad.origin.x as u16,
                    quad.origin.y as u16,
                ))?;
                self.terminal.write(full_line.as_bytes())?;
                (0..height - 2)
                    .into_iter()
                    .map(|y| {
                        display_blank(
                            &mut self.terminal,
                            quad.origin.x as u16,
                            quad.origin.y as u16 + 1 + y as u16,
                            &blank_line,
                        )
                    })
                    .collect::<Result<_, _>>()?;
                self.terminal.batch(Action::MoveCursorTo(
                    quad.origin.x as u16,
                    quad.origin.y as u16 + quad.size.height as u16 - 1,
                ))?;
                self.terminal.write(full_line.as_bytes())?;
            }
        }

        Ok(())
    }
}

impl Default for TerminalBackend {
    fn default() -> Self {
        let terminal = terminal::stdout();
        terminal
            .act(Action::ClearTerminal(Clear::All))
            .expect("Failed to clear terminal");
        terminal
            .act(Action::HideCursor)
            .expect("Failed to hide cursor");
        terminal
            .act(Action::EnableRawMode)
            .expect("Failed to enable raw mode");

        TerminalBackend {
            measure_brush: MeasureBrush {
                measure: Box::new(measure_text),
            },
            terminal,
        }
    }
}

#[derive(Debug)]
pub enum TerminalPrimitive {
    Common(CommonPrimitive<Self>),
    Text(TextPrimitive<(), Color>),
    Quad(QuadPrimitive<Color>),
}

fn measure_text(contents: &str, _font: (), _font_size: u16) -> Size {
    Size {
        width: contents.len() as f32,
        height: 1.,
    }
}

impl ContextProvider<MeasureBrush<()>> for TerminalBackend {
    fn provide(&self) -> &MeasureBrush<()> {
        &self.measure_brush
    }
}

impl ContextProvider<()> for TerminalBackend {
    fn provide(&self) -> &() {
        &()
    }
}

impl<A, B> From<(A, B)> for TerminalPrimitive
where
    TerminalPrimitive: From<A>,
    TerminalPrimitive: From<B>,
{
    fn from((a, b): (A, B)) -> Self {
        TerminalPrimitive::Common(CommonPrimitive::Group {
            children: vec![a.into(), b.into()],
        })
    }
}

impl From<QuadPrimitive<Color>> for TerminalPrimitive {
    fn from(quad: QuadPrimitive<Color>) -> TerminalPrimitive {
        TerminalPrimitive::Quad(quad)
    }
}

impl From<CommonPrimitive<Self>> for TerminalPrimitive {
    fn from(common: CommonPrimitive<Self>) -> TerminalPrimitive {
        TerminalPrimitive::Common(common)
    }
}

impl From<TextPrimitive<(), Color>> for TerminalPrimitive {
    fn from(text: TextPrimitive<(), ()>) -> TerminalPrimitive {
        TerminalPrimitive::Text(text)
    }
}

impl Backend for TerminalBackend {
    type Primitive = TerminalPrimitive;
}
