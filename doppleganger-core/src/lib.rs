use std::{borrow::Cow, collections::HashMap};

pub trait Doppleganger {
    type Source;
    type Dest;

    fn mirror(source: Self::Source) -> Self::Dest;
}

impl<T> Doppleganger for Vec<T>
where
    T: Doppleganger,
{
    type Source = Vec<T::Source>;

    type Dest = Vec<T::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source
            .into_iter()
            .map(<T as Doppleganger>::mirror)
            .collect()
    }
}

impl<T> Doppleganger for Option<T>
where
    T: Doppleganger,
{
    type Source = Option<T::Source>;

    type Dest = Option<T::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source.map(<T as Doppleganger>::mirror)
    }
}

impl<T, E> Doppleganger for Result<T, E>
where
    T: Doppleganger,
    E: Doppleganger,
{
    type Source = Result<T::Source, E::Source>;

    type Dest = Result<T::Dest, E::Dest>;

    #[inline]
    fn mirror(source: Self::Source) -> Self::Dest {
        source
            .map(<T as Doppleganger>::mirror)
            .map_err(<E as Doppleganger>::mirror)
    }
}

impl<K, V> Doppleganger for HashMap<K, V>
where
    K: Doppleganger,
    V: Doppleganger,
    K::Dest: Eq + std::hash::Hash,
{
    type Source = HashMap<K::Source, V::Source>;

    type Dest = HashMap<K::Dest, V::Dest>;

    fn mirror(source: Self::Source) -> Self::Dest {
        source
            .into_iter()
            .map(|(k, v)| {
                (
                    <K as Doppleganger>::mirror(k),
                    <V as Doppleganger>::mirror(v),
                )
            })
            .collect()
    }
}

pub trait Primitive: Sized {}

impl<T> Doppleganger for T
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
impl Primitive for bool {}

#[cfg(feature = "chrono")]
impl Primitive for chrono::DateTime<chrono::Utc> {}

#[cfg(feature = "uuid")]
impl Primitive for uuid::Uuid {}
