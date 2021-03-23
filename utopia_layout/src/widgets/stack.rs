use utopia_core::{
    math::{Size, Vector2},
    widgets::{pod::WidgetPod, TypedWidget, Widget},
    Backend, BoxConstraints, CommonPrimitive,
};

pub struct Stack<T, B: Backend> {
    children: Vec<WidgetPod<T, B>>,
}

impl<T, B: Backend> Stack<T, B> {
    pub fn new() -> Self {
        Stack {
            children: Vec::new(),
        }
    }

    pub fn add<TW: TypedWidget<T, B> + 'static>(mut self, child: TW) -> Self {
        self.children.push(WidgetPod::new(child));
        self
    }
}

impl<T, B: Backend> Widget<T> for Stack<T, B>
where
    B::Event: Clone,
{
    type Primitive = CommonPrimitive<B::Primitive>;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        let children: Vec<B::Primitive> = self
            .children
            .iter()
            .map(|c| TypedWidget::<T, B>::draw(c, origin, size, data))
            .collect();

        CommonPrimitive::Group { children }
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        self.children
            .iter_mut()
            .fold(Size::default(), |mut size, c| {
                let child_size = TypedWidget::<T, B>::layout(c, bc, context, data);
                size.width = size.width.max(child_size.width);
                size.height = size.height.max(child_size.height);
                size
            })
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.children
            .iter_mut()
            .rev()
            .filter_map(|c| TypedWidget::<T, B>::event(c, origin, size, data, event.clone()))
            .next()
    }
}
