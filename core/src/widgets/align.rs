use crate::math::{Size, Vector2};
use crate::widgets::pod::WidgetPod;
use crate::widgets::{TypedWidget, Widget};
use crate::{Backend, BoxConstraints};

pub struct Align<T, B: Backend> {
    widget: WidgetPod<T, B>,
    vertical: VerticalAlignment,
    horizontal: HorizontalAlignment,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Center
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        HorizontalAlignment::Center
    }
}

impl<T, B: Backend> Align<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(widget: TW) -> Self {
        Align {
            widget: WidgetPod::new(widget),
            vertical: VerticalAlignment::default(),
            horizontal: HorizontalAlignment::default(),
        }
    }
}

impl<T, B: Backend> Widget<T> for Align<T, B> {
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        let child_size = TypedWidget::<T, B>::layout(&mut self.widget, bc, context, data);

        let left = match self.horizontal {
            HorizontalAlignment::Left => 0.,
            HorizontalAlignment::Center => bc.max.width / 2. - child_size.width / 2.,
            HorizontalAlignment::Right => bc.max.width - child_size.width,
        };
        let top = match self.vertical {
            VerticalAlignment::Top => 0.,
            VerticalAlignment::Center => bc.max.height / 2. - child_size.height / 2.,
            VerticalAlignment::Bottom => bc.max.height - child_size.height,
        };

        self.widget.set_origin(Vector2 { x: left, y: top });

        Size {
            width: bc.max.width,
            height: bc.max.height,
        }
    }

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn event(&mut self, data: &mut T, event: &Self::Event) -> Option<Self::Reaction> {
        TypedWidget::<T, B>::event(&mut self.widget, data, event)
    }
}
