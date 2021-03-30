use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::{
    event::AnimateEvent,
    model::{AnimationRepeat, AnimationTarget},
};

use keyframe::{functions::Linear, CanTween, EasingFunction};
use utopia_core::{
    controllers::TransformEvent,
    lens::{Lens, NoLens},
    math::{Size, Vector2},
    reactions::CommonReaction,
    widgets::{TypedWidget, Widget},
    Backend, BoxConstraints,
};

/// Animates a property of a widget
///
/// * U is the type of the animated property
/// * L is a Lens that gets access to the animated property
/// * EF is an EasingFunction implementor
/// * TW is a type that dereferences into the wrapped Widget type
/// * W is the inner Widget
pub struct Animated<T, U, L, EF, TW, W, B, LTU = NoLens> {
    widget: TW,
    lens: L,
    easing_func: EF,
    starting_value: Option<U>,
    target: AnimationTarget<U, LTU>,
    repeat: AnimationRepeat,
    current_timer: Duration,
    anim_duration: Duration,
    _w: std::marker::PhantomData<W>,
    _t: std::marker::PhantomData<T>,
    _b: std::marker::PhantomData<B>,
}

impl<T, U, L, EF, TW: Deref<Target = W> + DerefMut, W, B: Backend, LTU> Deref
    for Animated<T, U, L, EF, TW, W, B, LTU>
{
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.widget.deref()
    }
}

impl<T, U, L, EF, TW: Deref<Target = W> + DerefMut, W, B: Backend, LTU> DerefMut
    for Animated<T, U, L, EF, TW, W, B, LTU>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.widget.deref_mut()
    }
}

impl<
        T,
        U: CanTween + Clone,
        L: Lens<W, U>,
        LTU: Lens<T, U>,
        EF: EasingFunction,
        TW: Deref<Target = W> + DerefMut + TypedWidget<T, B>,
        W,
        B: Backend,
    > Widget<T> for Animated<T, U, L, EF, TW, W, B, LTU>
where
    B::Event: TransformEvent<AnimateEvent> + Clone,
    B::EventReaction: From<CommonReaction>,
{
    type Primitive = B::Primitive;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn draw(&self, origin: Vector2, size: Size, data: &T) -> Self::Primitive {
        TypedWidget::<T, B>::draw(&self.widget, origin, size, data)
    }

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        TypedWidget::<T, B>::layout(&mut self.widget, bc, context, data)
    }

    fn event(
        &mut self,
        origin: Vector2,
        size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        let child_reaction =
            TypedWidget::<T, B>::event(&mut self.widget, origin, size, data, event.clone());
        let widget = self.widget.deref_mut();
        if let Some(AnimateEvent { elapsed }) = event.transform_event() {
            if self.starting_value.is_none() {
                self.starting_value =
                    Some(self.lens.with(widget, |widget_data| widget_data.clone()));
            }
            self.current_timer = (self.current_timer + elapsed).min(self.anim_duration);
            let time_factor = 1.
                - (self.anim_duration.as_secs_f64() - self.current_timer.as_secs_f64())
                    / self.anim_duration.as_secs_f64();
            let target_value = match &self.target {
                AnimationTarget::Fixed(data) => data.clone(),
                AnimationTarget::FromData(lens) => lens.with(data, |data| data.clone()),
            };

            let value = U::ease(
                self.starting_value.clone().unwrap(),
                target_value,
                self.easing_func.y(time_factor),
            );
            self.lens
                .with_mut(widget, |widget_data| *widget_data = value);
            return Some(CommonReaction::ReLayout.into());
        }
        child_reaction
    }
}

impl<
        T,
        U: CanTween + Clone,
        L: Lens<W, U>,
        TW: Deref<Target = W> + DerefMut + TypedWidget<T, B>,
        W,
        B: Backend,
    > Animated<T, U, L, Linear, TW, W, B>
{
    pub fn animate_to(widget: TW, lens: L, target: U) -> Self {
        Animated {
            widget,
            lens,
            repeat: AnimationRepeat::default(),
            anim_duration: Duration::from_secs_f32(1.),
            current_timer: Duration::default(),
            easing_func: Linear,
            target: AnimationTarget::Fixed(target),
            starting_value: None,
            _w: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _b: std::marker::PhantomData,
        }
    }

    pub fn animate_to_data<LTU: Lens<T, U>>(
        widget: TW,
        lens: L,
        target: LTU,
    ) -> Animated<T, U, L, Linear, TW, W, B, LTU> {
        Animated {
            widget,
            lens,
            repeat: AnimationRepeat::default(),
            anim_duration: Duration::default(),
            current_timer: Duration::default(),
            easing_func: Linear,
            target: AnimationTarget::FromData(target),
            starting_value: None,
            _w: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _b: std::marker::PhantomData,
        }
    }
}

impl<T, U, L, EF, TW, W, B: Backend> Animated<T, U, L, EF, TW, W, B> {
    pub fn easing<NEF: EasingFunction>(
        self,
        easing_function: NEF,
    ) -> Animated<T, U, L, NEF, TW, W, B> {
        let Animated {
            widget,
            lens,
            repeat,
            anim_duration,
            current_timer,
            target,
            starting_value,
            ..
        } = self;

        Animated {
            easing_func: easing_function,
            widget,
            lens,
            repeat,
            anim_duration,
            current_timer,
            target,
            starting_value,
            _w: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _b: std::marker::PhantomData,
        }
    }

    pub fn repeat(mut self, anim_repeat: AnimationRepeat) -> Self {
        self.repeat = anim_repeat;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.anim_duration = duration;
        self
    }
}
