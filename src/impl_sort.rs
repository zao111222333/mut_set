use crate::{Item, MutSet};
use core::hash::BuildHasher;

impl<T, S> MutSet<T, S>
where
    T: Item + Ord,
    S: BuildHasher,
{
    /// sort
    #[inline]
    pub fn sort(&mut self) {
        self.inner.sort_by(|_, a, _, b| a.cmp(b));
    }
}
