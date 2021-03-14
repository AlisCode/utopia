use std::collections::HashMap;

use utopia_core::{
    math::{Size, Vector2},
    Backend, BoxConstraints, CommonPrimitive,
};

use utopia_core::widgets::{pod::WidgetPod, TypedWidget, Widget};

/// A Widget that implements a Flex layout algorithm.
pub struct Flex<T, B: Backend> {
    children: Vec<FlexChild<T, B>>,
    computed_sizes: Vec<Size>,
}

struct FlexChild<T, B: Backend> {
    pub widget: WidgetPod<T, B>,
    pub flex_option: FlexOption,
}

enum FlexOption {
    NonFlex,
    Flex(u8),
}

impl<T, B: Backend> Default for Flex<T, B> {
    fn default() -> Self {
        Flex {
            children: Vec::default(),
            computed_sizes: Vec::default(),
        }
    }
}

impl<T, B: Backend> Flex<T, B> {
    pub fn add<TW: TypedWidget<T, B> + 'static>(&mut self, widget: TW) {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget),
            flex_option: FlexOption::NonFlex,
        });
        self.computed_sizes.push(Size::default());
    }

    pub fn add_flex<TW: TypedWidget<T, B> + 'static>(&mut self, widget: TW, flex_factor: u8) {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget),
            flex_option: FlexOption::Flex(flex_factor),
        });
        self.computed_sizes.push(Size::default());
    }
}

impl<T, B: Backend> Widget<T> for Flex<T, B>
where
    B::Event: Clone,
{
    type Primitive = CommonPrimitive<B::Primitive>;
    type Context = B;
    type Event = B::Event;
    type Reaction = B::EventReaction;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        // Step 1 : Layout inflexible children
        let loosened = bc.loosen();

        let inflexible_children: Vec<(usize, Size)> = self
            .children
            .iter_mut()
            .enumerate()
            .filter_map(|(index, child)| match child.flex_option {
                FlexOption::NonFlex => Some((
                    index,
                    TypedWidget::<T, B>::layout(&mut child.widget, &loosened, context, data),
                )),
                FlexOption::Flex(_) => None,
            })
            .collect();

        // Step 2 : Compute free space
        let width = bc.max.width;
        let sum_inflexible_children_width: f32 = inflexible_children
            .iter()
            .map(|(_index, size)| size.width)
            .sum();
        let free_space = width - sum_inflexible_children_width;
        let flex_factor_sum = self
            .children
            .iter()
            .filter_map(|child| match child.flex_option {
                FlexOption::Flex(flex) => Some(flex),
                FlexOption::NonFlex => None,
            })
            .sum::<u8>() as f32;
        let space_per_flex = free_space / flex_factor_sum;

        // Step 3 : Compute flexible children
        let flexible_children: Vec<(usize, Size)> = self
            .children
            .iter_mut()
            .enumerate()
            .filter_map(|(index, child)| match child.flex_option {
                FlexOption::Flex(factor) => {
                    let factor = factor as f32;
                    let constraint = BoxConstraints {
                        min: Size {
                            width: factor * space_per_flex,
                            height: 0.,
                        },
                        max: Size {
                            width: factor * space_per_flex,
                            height: bc.max.height,
                        },
                    };
                    Some((
                        index,
                        TypedWidget::<T, B>::layout(&mut child.widget, &constraint, context, data),
                    ))
                }
                FlexOption::NonFlex => None,
            })
            .collect();

        inflexible_children
            .iter()
            .chain(flexible_children.iter())
            .for_each(|(index, size)| {
                self.computed_sizes[*index] = *size;
            });

        let height = inflexible_children
            .iter()
            .chain(flexible_children.iter())
            .map(|(_index, size)| size.height.ceil() as u32)
            .max()
            .unwrap_or_default()
            .max(bc.min.height as u32) as f32;
        let index_and_size: HashMap<usize, Size> = inflexible_children
            .into_iter()
            .chain(flexible_children.into_iter())
            .collect();

        // Step 4: Position children
        let mut position = Vector2::ZERO;
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, child)| {
                child.widget.set_origin(position);
                let size = index_and_size[&index];
                position.x += size.width;
            });

        Size {
            width: position.x,
            height,
        }
    }

    fn draw(&self, origin: Vector2, _size: Size, data: &T) -> Self::Primitive {
        let children = self
            .children
            .iter()
            .zip(self.computed_sizes.iter())
            .map(|(flex_child, size)| {
                TypedWidget::<T, B>::draw(&flex_child.widget, origin, *size, data)
            })
            .collect();

        CommonPrimitive::Group { children }
    }

    fn event(
        &mut self,
        origin: Vector2,
        _size: Size,
        data: &mut T,
        event: Self::Event,
    ) -> Option<Self::Reaction> {
        self.children
            .iter_mut()
            .zip(self.computed_sizes.iter())
            .filter_map(|(flex_child, size)| {
                TypedWidget::<T, B>::event(
                    &mut flex_child.widget,
                    origin,
                    *size,
                    data,
                    event.clone(),
                )
            })
            .next()
    }
}