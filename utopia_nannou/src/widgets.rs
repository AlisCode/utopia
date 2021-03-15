use crate::font::Font;
use nannou::wgpu::Texture;
use utopia_core::{
    controllers::TypedController,
    lens::Lens,
    widgets::{
        controlled::Controlled as ControlledWidget, lens::LensWrap as LensWrapWidget,
        pod::WidgetPod, CoreExt, TypedWidget,
    },
};
use utopia_decorations::widgets::{border::Border as BorderWidget, DecorationsExt};
use utopia_image::widgets::image::Image as ImageWidget;
use utopia_layout::widgets::{
    align::Align as AlignWidget, flex::Flex as FlexWidget, padding::Padding as PaddingWidget,
    LayoutExt,
};
use utopia_text::widgets::text::Text as TextWidget;

use crate::NannouBackend;

pub type Align<T> = AlignWidget<T, NannouBackend>;
pub type Color = nannou::color::Srgb<u8>;
pub type Controlled<T, W, C> = ControlledWidget<T, W, C, NannouBackend>;
pub type Image = ImageWidget<Texture>;
pub type NannouWidgetPod<T> = WidgetPod<T, NannouBackend>;
pub type Flex<T> = FlexWidget<T, NannouBackend>;
pub type Text = TextWidget<Font, Color>;
pub type Border<T> = BorderWidget<T, Color, NannouBackend>;
pub type LensWrap<T, U, L> = LensWrapWidget<T, U, L, NannouBackend>;
pub type Padding<T> = PaddingWidget<T, NannouBackend>;

pub trait WidgetExt<T>: TypedWidget<T, NannouBackend> + Sized + 'static {
    // ----
    // LayoutExt
    // ----

    fn padding(self) -> Padding<T> {
        LayoutExt::<T, NannouBackend>::padding(self)
    }

    fn align(self) -> Align<T> {
        LayoutExt::<T, NannouBackend>::align(self)
    }

    fn centered(self) -> Align<T> {
        LayoutExt::<T, NannouBackend>::centered(self)
    }

    // ----
    // DecorationsExt
    // ----

    fn border(self) -> Border<T> {
        DecorationsExt::<T, NannouBackend>::bordered(self)
    }

    // ----
    // CoreExt
    // ----

    fn controlled<C: TypedController<T, Self, NannouBackend>>(
        self,
        controller: C,
    ) -> Controlled<T, Self, C> {
        CoreExt::<T, NannouBackend>::controlled(self, controller)
    }
}

pub trait LensExt<T>: Sized + 'static {
    fn lens<U, L: Lens<T, U>>(self, lens: L) -> LensWrap<T, U, L>
    where
        Self: TypedWidget<U, NannouBackend>,
    {
        LensWrap::new(self, lens)
    }
}

impl<T, W: 'static> LensExt<T> for W {}
impl<T, W: TypedWidget<T, NannouBackend> + Sized + 'static> WidgetExt<T> for W {}
