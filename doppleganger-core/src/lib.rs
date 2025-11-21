use std::{borrow::Cow, collections::HashMap};

pub trait Mirror {
    type Source;
    type Dest;

    fn mirror(source: Self::Source) -> Self::Dest;
}

impl<T> Mirror for Vec<T>
where
    T: Mirror,
{
    type Source = Vec<T::Source>;

    type Dest = Vec<T::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source.into_iter().map(<T as Mirror>::mirror).collect()
    }
}

impl<T> Mirror for Option<T>
where
    T: Mirror,
{
    type Source = Option<T::Source>;

    type Dest = Option<T::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source.map(<T as Mirror>::mirror)
    }
}

impl<T, E> Mirror for Result<T, E>
where
    T: Mirror,
    E: Mirror,
{
    type Source = Result<T::Source, E::Source>;

    type Dest = Result<T::Dest, E::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source
            .map(<T as Mirror>::mirror)
            .map_err(<E as Mirror>::mirror)
    }
}

impl<K, V> Mirror for HashMap<K, V>
where
    K: Mirror,
    V: Mirror,
    K::Dest: Eq + std::hash::Hash,
{
    type Source = HashMap<K::Source, V::Source>;

    type Dest = HashMap<K::Dest, V::Dest>;

    fn mirror(source: Self::Source) -> Self::Dest {
        source
            .into_iter()
            .map(|(k, v)| (<K as Mirror>::mirror(k), <V as Mirror>::mirror(v)))
            .collect()
    }
}

pub trait Primitive: Sized {}

impl<T> Mirror for T
where
    T: Primitive,
{
    type Source = Self;

    type Dest = Self;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source
    }
}

impl Primitive for String {}
impl Primitive for &str {}
impl<'a> Primitive for Cow<'a, str> {}
impl Primitive for isize {}
impl Primitive for i64 {}
impl Primitive for i32 {}
impl Primitive for i16 {}
impl Primitive for i8 {}
impl Primitive for usize {}
impl Primitive for u64 {}
impl Primitive for u32 {}
impl Primitive for u16 {}
impl Primitive for u8 {}
impl Primitive for f64 {}
impl Primitive for f32 {}
impl Primitive for bool {}

#[cfg(feature = "chrono")]
impl Primitive for chrono::DateTime<chrono::Utc> {}

#[cfg(feature = "uuid")]
impl Primitive for uuid::Uuid {}
