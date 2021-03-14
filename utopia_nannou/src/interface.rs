use nannou::{
    event::{MouseButton, WindowEvent},
    geom::Vector3,
    App, Event, Frame,
};
use utopia_core::{
    controllers::click::MouseClickEvent,
    interface::Interface,
    math::{Size, Vector2},
    widgets::TypedWidget,
};

use crate::{event::NannouEvent, NannouBackend};

pub struct NannouInterface<T> {
    interface: Interface<T, NannouBackend>,
    backend: NannouBackend,
    data: T,
    mouse_state: MouseState,
}

#[derive(Debug, Default)]
pub struct MouseState {
    pos: Vector2,
}

impl<T: 'static> NannouInterface<T> {
    pub fn new<W: TypedWidget<T, NannouBackend> + 'static>(
        widget: W,
        data: T,
        size: Size,
    ) -> NannouInterface<T> {
        let mut interface = Interface::new(widget);
        interface.resize(size);
        NannouInterface {
            interface,
            backend: NannouBackend::default(),
            data,
            mouse_state: MouseState::default(),
        }
    }

    fn view(app: &App, model: &Self, frame: Frame) {
        let primitive = model.interface.paint(&model.data);
        let size = app.window_rect();
        let draw = app
            .draw()
            .translate(Vector3::new(-size.w() / 2., -size.h() / 2., 0.));
        draw.background().color(nannou::color::WHITE);
        primitive.draw(&draw, size.h());
        draw.to_frame(app, &frame)
            .expect("Failed to write to frame");
    }

    fn event(app: &App, state: &mut Self, event: Event) {
        let win_rect = app.window_rect();
        match event {
            Event::Update(_update) => {
                state.interface.layout(&state.backend, &state.data);
            }
            Event::WindowEvent { simple, .. } => {
                if let Some(event) = simple {
                    match event {
                        WindowEvent::MouseMoved(pos) => {
                            state.mouse_state.pos =
                                Vector2::new(pos.x + win_rect.w() / 2., pos.y + win_rect.h() / 2.);
                        }
                        WindowEvent::MousePressed(button) => {
                            if button != MouseButton::Left {
                                return;
                            }
                            state
                                .interface
                                .add_event(NannouEvent::MouseClick(MouseClickEvent {
                                    mouse_button:
                                        utopia_core::controllers::click::MouseButton::Left,
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

        state.interface.event(&mut state.data);
    }

    pub fn run(callback: fn(&App) -> NannouInterface<T>) {
        nannou::app(callback)
            .event(Self::event)
            .simple_window(Self::view)
            .run();
    }
}
