use crate::{math::Vector2, widgets::Widget};

use super::Controller;

pub struct Click<T> {
    callback: Box<dyn Fn(&mut T)>,
}

impl<T> Click<T> {
    pub fn new<F: Fn(&mut T) + 'static>(on_click: F) -> Self {
        Click {
            callback: Box::new(on_click),
        }
    }
}

#[derive(Debug)]
pub struct MouseClickEvent {
    pos: Vector2,
    mouse_button: MouseButton,
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(usize),
}

impl<T, W: Widget<T>> Controller<T, W> for Click<T> {
    type Event = MouseClickEvent;
    type Reaction = ();

    fn event(
        &mut self,
        _child: &mut W,
        data: &mut T,
        event: &Self::Event,
    ) -> Option<Self::Reaction> {
        match event.mouse_button {
            MouseButton::Left => (self.callback)(data),
            _ => (),
        }

        None
    }
}
