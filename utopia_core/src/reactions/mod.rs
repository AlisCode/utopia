use crate::Backend;

pub enum CommonReaction {
    TakeFocus,
    ReDraw,
    ReLayout,
}

pub trait ReactionHandler<T> {
    type Reaction;

    fn handle(&mut self, reaction: Self::Reaction, data: &mut T);
}

pub trait TypedReactionHandler<T, B: Backend> {
    fn handle_reaction(
        &mut self,
        reaction: B::EventReaction,
        data: &mut T,
    ) -> Option<B::EventReaction>;
}

impl<B: Backend, T, RH: ReactionHandler<T>> TypedReactionHandler<T, B> for RH
where
    B::EventReaction: TransformReaction<RH::Reaction>,
{
    fn handle_reaction(
        &mut self,
        reaction: B::EventReaction,
        data: &mut T,
    ) -> Option<B::EventReaction> {
        match reaction.transform_reaction() {
            ReactionVariant::Casted(reac) => {
                self.handle(reac, data);
                None
            }
            ReactionVariant::Failed(failed) => Some(failed),
        }
    }
}

pub enum ReactionVariant<C, F> {
    Casted(C),
    Failed(F),
}

pub trait TransformReaction<Reaction>: Sized {
    fn transform_reaction(self) -> ReactionVariant<Reaction, Self>;
}

impl<T> TransformReaction<T> for T {
    fn transform_reaction(self) -> ReactionVariant<T, Self> {
        ReactionVariant::Casted(self)
    }
}
