use core::{
    math::Size,
    visitors::{layout::LayoutVisitor, paint::PaintVisitor, Visitor},
    widgets::{flex::Flex, text::Text},
    BoxConstraints, CommonPrimitive,
};
use empty::{TerminalBackend, TerminalPrimitive};
use terminal::{Action, Clear, Retrieved, Value};

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

    let mut column = Flex::<String, TerminalPrimitive, TerminalBackend>::default();
    column.add_flex(text, 1);
    column.add_flex(text_other, 1);

    let mut layout_visitor = LayoutVisitor {
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
    let size = <LayoutVisitor as Visitor<String, TerminalBackend>>::finish(layout_visitor);

    let mut paint_visitor = PaintVisitor {
        primitive: TerminalPrimitive::Common(CommonPrimitive::None),
        size,
    };

    paint_visitor.visit(&mut column, &backend, &data);
    let primitive = <PaintVisitor<TerminalPrimitive> as Visitor<String, TerminalBackend>>::finish(
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
