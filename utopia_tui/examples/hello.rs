use terminal::{Action, Clear, Retrieved, Value};
use utopia_core::{
    math::Size,
    steps::{layout::LayoutStep, paint::PaintStep, Visitor},
    BoxConstraints, CommonPrimitive,
};
use utopia_tui::{Align, Border, Flex, TerminalBackend, TerminalPrimitive, Text};

fn main() {
    let mut backend = TerminalBackend::default();

    let data = "Test".to_string();
    let text = Text {
        font: (),
        font_size: 16,
        color: (),
    };
    let text_other = Text {
        font: (),
        font_size: 20,
        color: (),
    };

    let size = backend
        .terminal
        .get(Value::TerminalSize)
        .expect("Failed to read terminal size");
    let max = match size {
        Retrieved::TerminalSize(width, height) => Size::new(width as f32, height as f32),
        _ => Size::new(0., 0.),
    };

    let mut column = Flex::default();
    column.add(text);
    column.add(Border::new(text_other));

    let mut column = Align::new(column);

    let mut layout_visitor = LayoutStep {
        box_constraints: BoxConstraints {
            min: Size {
                width: 0.,
                height: 0.,
            },
            max,
        },
        final_size: None,
    };
    layout_visitor.visit(&mut column, &backend, &data);
    let size = <LayoutStep as Visitor<String, TerminalBackend>>::finish(layout_visitor);

    let mut paint_visitor = PaintStep {
        primitive: TerminalPrimitive::Common(CommonPrimitive::None),
        size,
    };

    paint_visitor.visit(&mut column, &backend, &data);
    let primitive = <PaintStep<TerminalPrimitive> as Visitor<String, TerminalBackend>>::finish(
        paint_visitor,
    );

    backend
        .terminal
        .act(Action::ClearTerminal(Clear::All))
        .expect("Failed to clear terminal");
    backend
        .draw_primitive(primitive)
        .expect("Failed to draw primitives");
    backend.terminal.flush_batch().expect("Failed to flush");
    backend
        .terminal
        .get(Value::Event(None))
        .expect("Failed to get event");
}
