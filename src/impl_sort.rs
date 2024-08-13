use crate::{Item, MutSet};
use core::{cmp::Reverse, hash::BuildHasher};

impl<T, S> MutSet<T, S>
where
    T: Item + Ord,
    S: BuildHasher,
{
    /// into_iter_sort
    #[inline]
    pub fn into_iter_sort(self) -> impl Iterator<Item = T> {
        let mut vec = Vec::from_iter(
            self.inner
                .into_values()
                .map(<<T as Item>::ImmutIdItem as Into<T>>::into),
        );
        vec.sort();
        vec.into_iter()
    }
    /// iter_sort
    #[inline]
    pub fn iter_sort(&self) -> impl Clone + Iterator<Item = &T> {
        let mut vec = Vec::from_iter(
            self.inner
                .values()
                .map(<<T as Item>::ImmutIdItem as core::ops::Deref>::deref),
        );
        vec.sort();
        vec.into_iter()
    }

    /// into_iter_sort
    #[inline]
    pub fn into_iter_sort_reverse(self) -> impl Iterator<Item = T> {
        let mut vec = Vec::from_iter(
            self.inner
                .into_values()
                .map(|v| Reverse(<<T as Item>::ImmutIdItem as Into<T>>::into(v))),
        );
        vec.sort();
        vec.into_iter().map(|v| v.0)
    }
    /// iter_sort
    #[inline]
    pub fn iter_sort_reverse(&self) -> impl Clone + Iterator<Item = &T> {
        let mut vec =
            Vec::from_iter(self.inner.values().map(|v| {
                Reverse(<<T as Item>::ImmutIdItem as core::ops::Deref>::deref(v))
            }));
        vec.sort();
        vec.into_iter().map(|v| v.0)
    }
}
