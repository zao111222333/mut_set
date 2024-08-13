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
        let mut vec: Vec<T> = self.inner.into_values().collect();
        vec.sort();
        vec.into_iter()
    }
    /// iter_sort
    #[inline]
    pub fn iter_sort(&self) -> impl Clone + Iterator<Item = &T> {
        let mut vec: Vec<&T> = self.inner.values().collect();
        vec.sort();
        vec.into_iter()
    }
    /// into_iter_sort_reverse
    #[inline]
    pub fn into_iter_sort_reverse(self) -> impl Iterator<Item = T> {
        let mut vec: Vec<Reverse<T>> = self.inner.into_values().map(Reverse).collect();
        vec.sort();
        vec.into_iter().map(|v| v.0)
    }
    /// iter_sort_reverse
    #[inline]
    pub fn iter_sort_reverse(&self) -> impl Clone + Iterator<Item = &T> {
        let mut vec: Vec<Reverse<&T>> = self.inner.values().map(Reverse).collect();
        vec.sort();
        vec.into_iter().map(|v| v.0)
    }
}
