pub trait ContextProvider<Target> {
    fn provide(&self) -> &Target;
}

impl<T> ContextProvider<T> for T {
    fn provide(&self) -> &T {
        self
    }
}
