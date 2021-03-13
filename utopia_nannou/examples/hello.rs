use std::cell::RefCell;
use utopia_core::{
    controllers::click::{Click, MouseClickEvent},
    math::Size,
    visitors::{
        event::EventVisitor, layout::LayoutVisitor, paint::PaintVisitor, Visitor, VisitorMut,
    },
    BoxConstraints, CommonPrimitive,
};

use nannou::prelude::*;
use utopia_nannou::{
    Align, Border, Controlled, Flex, Font, LensWrap, NannouBackend, NannouEvent, NannouPrimitive,
    NannouWidgetPod, Padding, Text,
};

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct AppState<T> {
    backend: NannouBackend,
    state: T,
    widget: RefCell<NannouWidgetPod<T>>,
    widget_size: Option<Size>,
    event_visitor: EventVisitor<NannouEvent>,
    mouse_state: MouseState,
}

#[derive(Debug, Default)]
pub struct MouseState {
    pos: Vector2,
}

struct MyState {
    name: String,
    other_name: String,
}

fn view(app: &App, model: &AppState<MyState>, frame: Frame) {
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
        .translate(Vector3::new(-size.w() / 2., -size.h() / 2., 0.));
    draw.background().color(nannou::color::PLUM);
    primitive.draw(&draw, size.h());
    draw.to_frame(app, &frame)
        .expect("Failed to write to frame");
}

fn set_string(input: &mut String) {
    input.push('a');
}

fn model(_app: &App) -> AppState<MyState> {
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
    let text_other = Padding::new(text_other).all(5);
    let text_other = Border::new(text_other).border_width(5);

    let lens_name = utopia_core::lens!(MyState, name);
    let lens_name_other = utopia_core::lens!(MyState, other_name);

    let click = Click::new(set_string);
    let controlled = Controlled::new(text_other, click);

    let mut flex = Flex::default();
    flex.add(LensWrap::new(Border::new(text).border_width(5), lens_name));
    flex.add(LensWrap::new(controlled, lens_name_other));

    let centered = Align::new(flex);

    let state = MyState {
        name: "Test Nannou".to_string(),
        other_name: "My name".to_string(),
    };

    AppState {
        backend: NannouBackend::default(),
        state,
        widget: RefCell::new(NannouWidgetPod::new(centered)),
        widget_size: None,
        event_visitor: EventVisitor::default(),
        mouse_state: MouseState::default(),
    }
}

fn event(app: &App, state: &mut AppState<MyState>, event: Event) {
    let win_rect = app.window_rect();
    match event {
        Event::Update(_update) => {
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

            state.event_visitor.size = size;
            state
                .event_visitor
                .visit_mut(&mut *widget, &state.backend, &mut state.state);

            state.widget_size = Some(size);
        }
        Event::WindowEvent { simple, .. } => {
            if let Some(event) = simple {
                match event {
                    WindowEvent::MouseMoved(pos) => {
                        state.mouse_state.pos =
                            Point2::new(pos.x + win_rect.w() / 2., pos.y + win_rect.h() / 2.);
                    }
                    WindowEvent::MousePressed(button) => {
                        if button != MouseButton::Left {
                            return;
                        }
                        state
                            .event_visitor
                            .queue_event(NannouEvent::MouseClick(MouseClickEvent {
                                mouse_button: utopia_core::controllers::click::MouseButton::Left,
                                pos: utopia_core::math::Vector2::new(
                                    state.mouse_state.pos.x,
                                    win_rect.h() - state.mouse_state.pos.y,
                                ),
                            }))
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}
