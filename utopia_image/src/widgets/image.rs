use utopia_core::widgets::Widget;

use crate::{context::ImageContext, primitive::ImagePrimitive};

#[derive(Debug)]
pub struct Image<Img> {
    _src: std::marker::PhantomData<Img>,
}

impl<Img> Image<Img> {
    pub fn new() -> Self {
        Image {
            _src: std::marker::PhantomData,
        }
    }
}

impl<Img: Clone> Widget<Img> for Image<Img> {
    type Primitive = ImagePrimitive<Img>;
    type Context = ImageContext<Img>;
    type Event = ();
    type Reaction = ();

    fn draw(
        &self,
        origin: utopia_core::math::Vector2,
        size: utopia_core::math::Size,
        data: &Img,
    ) -> Self::Primitive {
        ImagePrimitive {
            position: origin,
            size,
            src: data.clone(),
        }
    }

    fn layout(
        &mut self,
        bc: &utopia_core::BoxConstraints,
        context: &Self::Context,
        data: &Img,
    ) -> utopia_core::math::Size {
        let size = (context.measure)(data);
        bc.constrain(size)
    }
}
