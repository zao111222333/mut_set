use crate::{Item, MutSet};
use core::hash::BuildHasher;

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
        self.inner
            .values()
            .all(|item| match other.inner.get(&other.id(item)) {
                Some(_item) => _item.eq(item),
                None => false,
            })
    }
}
