use utopia_core::controllers::{click::MouseClickEvent, TransformEvent};

#[derive(Debug, Clone)]
pub enum NannouEvent {
    MouseClick(MouseClickEvent),
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
        }
    }
}
