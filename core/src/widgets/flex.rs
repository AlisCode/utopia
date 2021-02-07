use std::collections::HashMap;

use crate::{
    contexts::ContextProvider,
    math::{Size, Vector2},
    BoxConstraints, CommonPrimitive,
};

use super::{pod::WidgetPod, Widget, WidgetExt};

pub struct Flex<T, P, C> {
    children: Vec<FlexChild<T, P, C>>,
}

struct FlexChild<T, P, C> {
    pub widget: WidgetPod<T, P, C>,
    pub flex_option: FlexOption,
}

enum FlexOption {
    NonFlex,
    Flex(u8),
}

impl<T, P, C> Default for Flex<T, P, C> {
    fn default() -> Self {
        Flex {
            children: Vec::default(),
        }
    }
}

impl<T: 'static, P: 'static, C: 'static> Flex<T, P, C> {
    pub fn add<W>(&mut self, widget: W)
    where
        W: Widget<T> + 'static,
        P: From<W::Primitive>,
        C: ContextProvider<W::Context>,
    {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget.map_primitive::<P>().map_context::<C>()),
            flex_option: FlexOption::NonFlex,
        })
    }

    pub fn add_flex<W>(&mut self, widget: W, flex_factor: u8)
    where
        W: Widget<T> + 'static,
        P: From<W::Primitive>,
        C: ContextProvider<W::Context>,
    {
        self.children.push(FlexChild {
            widget: WidgetPod::new(widget.map_primitive::<P>().map_context::<C>()),
            flex_option: FlexOption::Flex(flex_factor),
        })
    }
}

impl<T, P, C> Widget<T> for Flex<T, P, C> {
    type Primitive = CommonPrimitive<P>;
    type Context = C;

    fn layout(&mut self, bc: &BoxConstraints, context: &Self::Context, data: &T) -> Size {
        // Step 1 : Layout inflexible children
        let loosened = bc.loosen();

        let inflexible_children: Vec<(usize, Size)> = self
            .children
            .iter_mut()
            .enumerate()
            .filter_map(|(index, child)| match child.flex_option {
                FlexOption::NonFlex => Some((index, child.widget.layout(&loosened, context, data))),
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
                    Some((index, child.widget.layout(&constraint, context, data)))
                }
                FlexOption::NonFlex => None,
            })
            .collect();

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

        Size { width, height }
    }

    fn draw(&self, origin: Vector2, data: &T) -> Self::Primitive {
        let children = self
            .children
            .iter()
            .map(|flex_child| flex_child.widget.draw(origin, data))
            .collect();

        CommonPrimitive::Group { children }
    }
}
