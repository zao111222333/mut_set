use crate::{Item, MutSet};
use core::hash::BuildHasher;
use std::{collections::HashSet, ops::Deref};

impl<T, S> PartialEq for MutSet<T, S>
where
    T: Item + Eq,
    S: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }
        let set: HashSet<&T> = self.inner.values().map(Deref::deref).collect();
        self.inner.values().all(|immut_item| {
            let item = immut_item.deref();
            match set.get(item) {
                Some(&_item) => _item.eq(item),
                None => false,
            }
        })
    }
}
