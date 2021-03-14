use crate::font::Font;
use utopia_core::widgets::{
    controlled::Controlled as ControlledWidget, lens::LensWrap as LensWrapWidget, pod::WidgetPod,
};
use utopia_decorations::widgets::border::Border as BorderWidget;
use utopia_layout::widgets::{
    align::Align as AlignWidget, flex::Flex as FlexWidget, padding::Padding as PaddingWidget,
};
use utopia_text::widgets::text::Text as TextWidget;

use crate::NannouBackend;

pub type Align<T> = AlignWidget<T, NannouBackend>;
pub type Color = nannou::color::Srgb<u8>;
pub type Controlled<T, W, C> = ControlledWidget<T, W, C, NannouBackend>;
pub type NannouWidgetPod<T> = WidgetPod<T, NannouBackend>;
pub type Flex<T> = FlexWidget<T, NannouBackend>;
pub type Text = TextWidget<Font, Color>;
pub type Border<T> = BorderWidget<T, Color, NannouBackend>;
pub type LensWrap<T, U, L> = LensWrapWidget<T, U, L, NannouBackend>;
pub type Padding<T> = PaddingWidget<T, NannouBackend>;
