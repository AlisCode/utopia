#[derive(Debug)]
pub enum AnimationRepeat {
    /// Loop the animation one way
    Loop,
    /// The animation doesn't loop
    Once,
    /// Loop the animation, doing it one-way, then coming back  
    PingPong,
}

impl Default for AnimationRepeat {
    fn default() -> Self {
        AnimationRepeat::Once
    }
}

pub enum AnimationTarget<U, LTU> {
    Fixed(U),
    FromData(LTU),
}
