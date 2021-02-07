use core::{
    contexts::ContextProvider,
    math::Size,
    widgets::text::{MeasureBrush, TextPrimitive},
    Backend, CommonPrimitive,
};
use std::io::{Stdout, Write};

use terminal::{Action, Clear, Terminal};
pub struct TerminalBackend {
    measure_brush: MeasureBrush<()>,
    pub terminal: Terminal<Stdout>,
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
    Text(TextPrimitive<(), ()>),
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

impl From<CommonPrimitive<Self>> for TerminalPrimitive {
    fn from(common: CommonPrimitive<Self>) -> TerminalPrimitive {
        TerminalPrimitive::Common(common)
    }
}

impl From<TextPrimitive<(), ()>> for TerminalPrimitive {
    fn from(text: TextPrimitive<(), ()>) -> TerminalPrimitive {
        TerminalPrimitive::Text(text)
    }
}

impl Backend for TerminalBackend {
    type Primitive = TerminalPrimitive;
}
