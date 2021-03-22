use std::ops::{Deref, DerefMut};

use crate::font::Font;
use nannou::wgpu::Texture;
use utopia_core::{
    controllers::TypedController,
    lens::Lens,
    widgets::{
        controlled::Controlled as ControlledWidget, lens::LensWrap as LensWrapWidget,
        pod::WidgetPod, styled::Styled as StyledWidget, CoreExt, TypedWidget,
    },
};
use utopia_decorations::widgets::{
    background::Background as BackgroundWidget, border::Border as BorderWidget, DecorationsExt,
};
use utopia_image::widgets::image::Image as ImageWidget;
use utopia_layout::{
    widgets::{
        align::Align as AlignWidget, flex::Flex as FlexWidget, max_size::MaxSize as MaxSizeWidget,
        min_size::MinSize as MinSizeWidget, padding::Padding as PaddingWidget, LayoutExt,
    },
    SizeConstraint,
};
use utopia_scroll::widgets::scrollview::ScrollView as ScrollViewWidget;
use utopia_text::widgets::{label::Label as LabelWidget, text::Text as TextWidget};

use crate::NannouBackend;

pub type Align<T> = AlignWidget<T, NannouBackend>;
pub type Color = nannou::color::Srgb<u8>;
pub type Controlled<T, W, C> = ControlledWidget<T, W, C, NannouBackend>;
pub type Image = ImageWidget<Texture>;
pub type NannouWidgetPod<T> = WidgetPod<T, NannouBackend>;
pub type Flex<T> = FlexWidget<T, NannouBackend>;
pub type Text = TextWidget<Font, Color>;
pub type Label = LabelWidget<Font, Color>;
pub type Border<T> = BorderWidget<T, Color, NannouBackend>;
pub type Background<T> = BackgroundWidget<T, Color, NannouBackend>;
pub type LensWrap<T, U, L, W> = LensWrapWidget<T, U, L, W, NannouBackend>;
pub type Padding<T> = PaddingWidget<T, NannouBackend>;
pub type MinSize<T> = MinSizeWidget<T, NannouBackend>;
pub type MaxSize<T> = MaxSizeWidget<T, NannouBackend>;
pub type Styled<U, L, LW, W, TW> = StyledWidget<U, L, LW, W, TW, NannouBackend>;
pub type ScrollView<T> = ScrollViewWidget<T, NannouBackend>;

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

    fn min_size(self, constraint: SizeConstraint) -> MinSize<T> {
        MinSize::new(self, constraint)
    }

    fn max_size(self, constraint: SizeConstraint) -> MaxSize<T> {
        MaxSize::new(self, constraint)
    }

    // ----
    // ScrollExt
    // ----

    fn scroll(self) -> ScrollView<T> {
        ScrollView::new(self)
    }

    // ----
    // DecorationsExt
    // ----

    fn border(self) -> Border<T> {
        DecorationsExt::<T, NannouBackend>::border(self)
    }

    fn background(self) -> Background<T> {
        DecorationsExt::<T, NannouBackend>::background(self)
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

    fn styled<U: Clone, W, L: Lens<T, U>, LW: Lens<W, U>>(
        self,
        lens: L,
        lens_widget: LW,
    ) -> Styled<U, L, LW, W, Self>
    where
        Self: Deref<Target = W> + DerefMut,
    {
        Styled::new::<T>(self, lens, lens_widget)
    }
}

pub trait LensExt<T>: Sized + 'static {
    fn lens<U, L: Lens<T, U>>(self, lens: L) -> LensWrap<T, U, L, Self>
    where
        Self: TypedWidget<U, NannouBackend>,
    {
        LensWrap::new(self, lens)
    }
}

impl<T, W: 'static> LensExt<T> for W {}
impl<T, W: TypedWidget<T, NannouBackend> + Sized + 'static> WidgetExt<T> for W {}
