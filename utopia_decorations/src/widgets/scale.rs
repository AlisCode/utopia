use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend,
};

pub struct Scale<T, B: Backend> {
    pub scale_x: f32,
    pub scale_y: f32,
    widget: WidgetPod<T, B>,
}

impl<T, B: Backend> Scale<T, B> {
    pub fn new<TW: TypedWidget<T, B> + 'static>(widget: TW) -> Self {
        Scale {
            scale_x: 1.,
            scale_y: 1.,
            widget: WidgetPod::new(widget),
        }
    }

    pub fn x(mut self, scale_x: f32) -> Self {
        self.scale_x = scale_x;
        self
    }

    pub fn y(mut self, scale_y: f32) -> Self {
        self.scale_y = scale_y;
        self
    }
}

#[derive(Debug)]
pub struct ScaledPrimitive<P> {
    pub scale_x: f32,
    pub scale_y: f32,
    pub origin: Vector2,
    pub primitive: Box<P>,
}

impl<T, B: Backend> Widget<T> for Scale<T, B> {
    type Primitive = ScaledPrimitive<B::Primitive>;
    type Context = B;
    type Reaction = B::EventReaction;
    type Event = B::Event;

    fn draw(
        &self,
        origin: utopia_core::math::Vector2,
        size: utopia_core::math::Size,
        data: &T,
    ) -> Self::Primitive {
        let primitive = TypedWidget::<T, B>::draw(&self.widget, origin, size, data);
        ScaledPrimitive {
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            origin,
            primitive: Box::new(primitive),
        }
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        TypedWidget::<T, B>::event(&mut self.widget, origin, size, data, event)
    }

    fn layout(
        &mut self,
        bc: &utopia_core::BoxConstraints,
        context: &Self::Context,
        data: &T,
    ) -> utopia_core::math::Size {
        TypedWidget::<T, B>::layout(&mut self.widget, bc, context, data)
    }
}
