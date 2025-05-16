use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};
use indexmap::{IndexSet, set::Iter};

use crate::{Item, MutSetExt};

impl<T: Item, S: BuildHasher> MutSetExt<T> for IndexSet<T, S> {
    type IterMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    fn get_mut<Q>(&mut self, value: &Q) -> Option<&mut T::ImmutIdItem>
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.get(value).map(Item::__unsafe_deref_mut)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        IterMut { inner: self.iter() }
    }
}

pub struct IterMut<'a, T: Item> {
    inner: Iter<'a, T>,
}

impl<'a, T: Item> Iterator for IterMut<'a, T> {
    type Item = &'a mut T::ImmutIdItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Item::__unsafe_deref_mut)
    }
}
