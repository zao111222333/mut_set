#![doc = include_str!("../README.md")]

pub mod derive {
    pub use mut_set_derive::item;
}

mod impl_hashset;
mod impl_indexmap;
use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
    ops::Deref,
};

/// Extend  `HashSet`/`IndexSet` with `get_mut`/`iter_mut`
pub trait MutSetExt<T: Item> {
    type IterMut<'a>
    where
        Self: 'a;
    fn get_mut<Q>(&mut self, value: &Q) -> Option<&mut T::IdReadonlyItem>
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

pub trait Item
where
    Self: Sized + Eq + Hash + Borrow<Self::Id>,
{
    type Id;
    type IdReadonlyItem: Deref<Target = Self>;
    fn id(&self) -> &Self::Id {
        self.borrow()
    }
    fn id_readonly(&mut self) -> &mut Self::IdReadonlyItem {
        unsafe { self.__unsafe_deref_mut() }
    }
    /// # Safety
    ///
    /// Only for internal usages
    #[expect(clippy::mut_from_ref)]
    unsafe fn __unsafe_deref_mut(&self) -> &mut Self::IdReadonlyItem;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoHashBuildHasher;
impl BuildHasher for NoHashBuildHasher {
    type Hasher = NoHashHasher;
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        NoHashHasher(0)
    }
}
pub struct NoHashHasher(u64);
impl core::hash::Hasher for NoHashHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }
    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }
}
