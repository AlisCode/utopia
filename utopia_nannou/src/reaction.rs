use utopia_core::reactions::CommonReaction;

pub enum NannouReaction {
    Common(CommonReaction),
    None,
}

impl From<()> for NannouReaction {
    fn from(_input: ()) -> Self {
        NannouReaction::None
    }
}

impl From<CommonReaction> for NannouReaction {
    fn from(input: CommonReaction) -> Self {
        NannouReaction::Common(input)
    }
}
