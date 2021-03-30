use std::ops::{Deref, DerefMut};

use keyframe::{functions::Linear, CanTween};
use utopia_core::{lens::Lens, widgets::TypedWidget, Backend};

use self::animated::Animated;

pub mod animated;

pub trait AnimationExt<T, B: Backend>: Sized + TypedWidget<T, B> + Deref + DerefMut {
    fn animate<L: Lens<<Self as Deref>::Target, U>, U: Clone + CanTween>(
        self,
        lens: L,
        target: U,
    ) -> Animated<T, U, L, Linear, Self, <Self as Deref>::Target, B>
    where
        <Self as Deref>::Target: Sized,
    {
        Animated::animate(self, lens, target)
    }

    fn animate_from_data<
        L: Lens<<Self as Deref>::Target, U>,
        LTU: Lens<T, U>,
        U: Clone + CanTween,
    >(
        self,
        lens: L,
        target: LTU,
    ) -> Animated<T, U, L, Linear, Self, <Self as Deref>::Target, B, LTU>
    where
        <Self as Deref>::Target: Sized,
    {
        Animated::animate_from_data(self, lens, target)
    }
}

impl<T, B: Backend, TW: Sized + TypedWidget<T, B> + Deref + DerefMut> AnimationExt<T, B> for TW where
    <TW as Deref>::Target: Sized
{
}
