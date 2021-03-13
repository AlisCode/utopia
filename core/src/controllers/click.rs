use crate::{
    math::{Size, Vector2},
    widgets::Widget,
};

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

#[derive(Debug, Clone)]
pub struct MouseClickEvent {
    pub pos: Vector2,
    pub mouse_button: MouseButton,
}

#[derive(Debug, Clone)]
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
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        let target = event.pos - origin;
        // Fix origin
        println!("target {:?} : {:?} - {:?}", target, event.pos, origin);
        //println!("checking contains {:?} in {:?}", target, size);
        if !size.contains(target) {
            return None;
        }

        match event.mouse_button {
            MouseButton::Left => (self.callback)(data),
            _ => (),
        }

        None
    }
}
