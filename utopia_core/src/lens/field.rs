use super::Lens;

/// Lens accessing a member of some type using accessor functions
///
/// See also the `lens` macro.
///
/// ```
/// let lens = $crate::lens::Field::new(|x: &Vec<u32>| &x[42], |x| &mut x[42]);
/// ```
pub struct Field<Get, GetMut> {
    get: Get,
    get_mut: GetMut,
}

impl<Get, GetMut> Field<Get, GetMut> {
    /// Construct a lens from a pair of getter functions
    pub fn new<T: ?Sized, U: ?Sized>(get: Get, get_mut: GetMut) -> Self
    where
        Get: Fn(&T) -> &U,
        GetMut: Fn(&mut T) -> &mut U,
    {
        Self { get, get_mut }
    }
}

impl<T, U, Get, GetMut> Lens<T, U> for Field<Get, GetMut>
where
    Get: Fn(&T) -> &U,
    GetMut: Fn(&mut T) -> &mut U,
{
    fn with<V, F: FnOnce(&U) -> V>(&self, data: &T, f: F) -> V {
        f((self.get)(data))
    }

    fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, data: &mut T, f: F) -> V {
        f((self.get_mut)(data))
    }
}

/// Construct a lens accessing a type's field
///
/// This is a convenience macro for constructing `Field` lenses for fields or indexable elements.
///
/// ```
/// struct Foo { x: u32 }
/// let lens = druid::lens!(Foo, x);
/// let lens = druid::lens!((u32, bool), 1);
/// let lens = druid::lens!([u8], [4]);
/// ```
#[macro_export]
macro_rules! lens {
    ($ty:ty, [$index:expr]) => {
        $crate::lens::Field::new::<$ty, _>(move |x| &x[$index], move |x| &mut x[$index])
    };
    ($ty:ty, $field:tt) => {
        $crate::lens::Field::new::<$ty, _>(move |x| &x.$field, move |x| &mut x.$field)
    };
}
