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
    flex_direction: FlexDirection,
}

#[derive(Clone, Copy)]
enum FlexDirection {
    Row,
    Column,
}

struct FlexChild<T, B: Backend> {
    pub widget: WidgetPod<T, B>,
    pub flex_option: FlexOption,
}

enum FlexOption {
    NonFlex,
    Flex(u8),
}

impl<T, B: Backend> Flex<T, B> {
    pub fn row() -> Self {
        Flex {
            children: Vec::default(),
            computed_sizes: Vec::default(),
            flex_direction: FlexDirection::Row,
        }
    }

    pub fn column() -> Self {
        Flex {
            children: Vec::default(),
            computed_sizes: Vec::default(),
            flex_direction: FlexDirection::Column,
        }
    }

    pub fn add<TW: TypedWidget<T, B> + 'static>(mut self, widget: TW) -> Self {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget),
            flex_option: FlexOption::NonFlex,
        });
        self.computed_sizes.push(Size::default());
        self
    }

    pub fn add_flex<TW: TypedWidget<T, B> + 'static>(
        mut self,
        widget: TW,
        flex_factor: u8,
    ) -> Self {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget),
            flex_option: FlexOption::Flex(flex_factor),
        });
        self.computed_sizes.push(Size::default());
        self
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
        let flex_direction = self.flex_direction;

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
        let space = match flex_direction {
            FlexDirection::Row => bc.max.width,
            FlexDirection::Column => bc.max.height,
        };
        let sum_inflexible_children: f32 = inflexible_children
            .iter()
            .map(|(_index, size)| match flex_direction {
                FlexDirection::Column => size.height,
                FlexDirection::Row => size.width,
            })
            .sum();
        let free_space = space - sum_inflexible_children;
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
                    let constraint = match flex_direction {
                        FlexDirection::Row => BoxConstraints {
                            min: Size {
                                width: factor * space_per_flex,
                                height: 0.,
                            },
                            max: Size {
                                width: factor * space_per_flex,
                                height: bc.max.height,
                            },
                        },
                        FlexDirection::Column => BoxConstraints {
                            min: Size {
                                width: 0.,
                                height: factor * space_per_flex,
                            },
                            max: Size {
                                width: bc.max.width,
                                height: factor * space_per_flex,
                            },
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

        // Update all computed sizes
        inflexible_children
            .iter()
            .chain(flexible_children.iter())
            .for_each(|(index, size)| {
                self.computed_sizes[*index] = *size;
            });

        let liner = inflexible_children
            .iter()
            .chain(flexible_children.iter())
            .map(|(_index, size)| match flex_direction {
                FlexDirection::Row => size.height.ceil() as u32,
                FlexDirection::Column => size.width.ceil() as u32,
            })
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
                match flex_direction {
                    FlexDirection::Row => position.x += size.width,
                    FlexDirection::Column => position.y += size.height,
                }
            });

        match flex_direction {
            FlexDirection::Row => Size {
                width: position.x,
                height: liner,
            },
            FlexDirection::Column => Size {
                width: liner,
                height: position.y,
            },
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
