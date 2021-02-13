use core::{
    math::Size,
    visitors::{layout::LayoutVisitor, paint::PaintVisitor, Visitor},
    BoxConstraints, CommonPrimitive,
};
use std::cell::RefCell;

use nannou::prelude::*;
use testui_nannou::{Border, Flex, Font, NannouBackend, NannouPrimitive, NannouWidgetPod, Text};

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct AppState<T> {
    backend: NannouBackend,
    state: T,
    widget: RefCell<NannouWidgetPod<T>>,
    widget_size: Option<Size>,
}

fn view(app: &App, model: &AppState<String>, frame: Frame) {
    let mut paint_visitor = PaintVisitor {
        size: model.widget_size.unwrap(),
        primitive: NannouPrimitive::Common(CommonPrimitive::None),
    };
    let mut widget = model.widget.borrow_mut();
    paint_visitor.visit(&mut *widget, &model.backend, &model.state);
    let primitive =
        <PaintVisitor<NannouPrimitive> as Visitor<String, NannouBackend>>::finish(paint_visitor);

    let size = app.window_rect();
    let draw = app
        .draw()
        .translate(Vector3::new(-size.w() / 2., size.h() / 2., 0.));
    draw.background().color(nannou::color::PLUM);
    primitive.draw(&draw);
    draw.to_frame(app, &frame)
        .expect("Failed to write to frame");
}

fn model(_app: &App) -> AppState<String> {
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
    let mut flex = Flex::default();
    flex.add(Border::new(text).border_width(10));
    flex.add(text_other);

    AppState {
        backend: NannouBackend::default(),
        state: "Test Nannou".to_string(),
        widget: RefCell::new(NannouWidgetPod::new(flex)),
        widget_size: None,
    }
}

fn event(app: &App, state: &mut AppState<String>, event: Event) {
    match event {
        Event::Update(_update) => {
            let win_rect = app.window_rect();
            let box_constraints = BoxConstraints {
                min: Size {
                    width: 0.,
                    height: 0.,
                },
                max: Size {
                    width: win_rect.w(),
                    height: win_rect.h(),
                },
            };
            let mut layout_visitor = LayoutVisitor {
                box_constraints,
                final_size: None,
            };
            let mut widget = state.widget.borrow_mut();
            layout_visitor.visit(&mut *widget, &state.backend, &state.state);
            let size = <LayoutVisitor as Visitor<String, NannouBackend>>::finish(layout_visitor);
            state.widget_size = Some(size);
        }
        _ => (),
    }
}
