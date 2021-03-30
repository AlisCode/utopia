use super::Lens;

pub struct NoLens;

impl<T, U> Lens<T, U> for NoLens {
    fn with<V, F: FnOnce(&U) -> V>(&self, _data: &T, _f: F) -> V {
        panic!("Should not be used");
    }

    fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, _data: &mut T, _f: F) -> V {
        panic!("Should not be used");
    }
}
