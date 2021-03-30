use std::time::Duration;

use utopia_animations::event::AnimateEvent;
use utopia_core::controllers::{click::MouseClickEvent, TransformEvent};

#[derive(Debug, Clone)]
pub enum NannouEvent {
    MouseClick(MouseClickEvent),
    Update(Duration),
}

impl TransformEvent<()> for NannouEvent {
    fn transform_event(self) -> Option<()> {
        Some(())
    }
}

impl TransformEvent<MouseClickEvent> for NannouEvent {
    fn transform_event(self) -> Option<MouseClickEvent> {
        match self {
            NannouEvent::MouseClick(click) => Some(click),
            _ => None,
        }
    }
}

impl TransformEvent<AnimateEvent> for NannouEvent {
    fn transform_event(self) -> Option<AnimateEvent> {
        match self {
            NannouEvent::Update(elapsed) => Some(AnimateEvent::new(elapsed)),
            _ => None,
        }
    }
}
