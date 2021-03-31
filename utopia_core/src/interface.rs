use crate::{
    math::Size,
    reactions::TypedReactionHandler,
    steps::{event::EventStep, layout::LayoutStep, paint::PaintStep},
    widgets::{pod::WidgetPod, TypedWidget},
    Backend, BoxConstraints,
};

pub struct Interface<T, B: Backend> {
    widget: WidgetPod<T, B>,
    event_step: EventStep<B::Event, B::EventReaction>,
    paint_step: PaintStep<B::Primitive>,
    layout_step: LayoutStep,
    reaction_handlers: Vec<Box<dyn TypedReactionHandler<T, B>>>,
}

impl<T, B: Backend> Interface<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(widget: TW) -> Self {
        Interface {
            widget: WidgetPod::new(widget),
            event_step: EventStep::default(),
            paint_step: PaintStep::default(),
            layout_step: LayoutStep::default(),
            reaction_handlers: Vec::default(),
        }
    }

    pub fn set_widget<W: TypedWidget<T, B> + 'static>(&mut self, widget: W) {
        self.widget = WidgetPod::new(widget)
    }

    pub fn add_reaction_handler<TRH: TypedReactionHandler<T, B> + 'static>(
        &mut self,
        reaction_handler: TRH,
    ) {
        self.reaction_handlers.push(Box::new(reaction_handler));
    }

    pub fn add_event(&mut self, event: B::Event) {
        self.event_step.queue_event(event)
    }

    pub fn resize(&mut self, new_size: Size) {
        self.layout_step.box_constraints = BoxConstraints {
            min: Size::default(),
            max: new_size,
        }
    }

    pub fn event(&mut self, data: &mut T) {
        self.event_step.apply::<T, B, _>(&mut self.widget, data);
        let reactions = self.event_step.drain_reactions();

        for reac in reactions {
            let mut r = Some(reac);
            for handler in self.reaction_handlers.iter_mut() {
                match r {
                    Some(reac) => r = handler.handle_reaction(reac, data),
                    None => break,
                }
            }
        }
    }

    pub fn layout(&mut self, backend: &B, data: &T) {
        let size = self.layout_step.apply(&mut self.widget, backend, data);
        self.paint_step.size = size;
    }

    pub fn paint(&self, data: &T) -> B::Primitive {
        self.paint_step.apply::<T, B, _>(&self.widget, data)
    }
}
