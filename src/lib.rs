//! See more at [![github](https://img.shields.io/badge/github-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
//!
//!```
//!#[derive(Debug)]
//!#[mut_set::derive::item]
//!pub struct MyItem<T1, T2>
//!where
//!    T1: Sized,
//!{
//!    #[id]
//!    pub(self) id1: usize,
//!    pub(crate) ctx1: T1,
//!    pub ctx2: T2,
//!    #[id]
//!    pub id2: String,
//!}
//!fn test() {
//!    let mut set = mut_set::MutSet::new();
//!    println!("{:?}", set);
//!    set.insert(MyItem {
//!        id1: 2,
//!        id2: "www".to_string(),
//!        ctx1: -1,
//!        ctx2: "ccc".to_string(),
//!    });
//!    set.insert(MyItem {
//!        id1: 1,
//!        id2: "ww".to_string(),
//!        ctx1: -2,
//!        ctx2: "cc".to_string(),
//!    });
//!    println!("{:?}", set);
//!    for v in set.iter() {
//!        println!("{:?}", v);
//!    }
//!    for v in set.iter_mut() {
//!        v.ctx1 = 0;
//!        println!("{:?}", v.id1);
//!        // In `iter_mut` IDs write will be prohibited
//!        // v.id1 = 0;
//!    }
//!    println!("{:?}", set);
//!    println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string())));
//!    set.replace(MyItem {
//!        id1: 1,
//!        id2: "ww".to_string(),
//!        ctx1: -2,
//!        ctx2: "cc".to_string(),
//!    });
//!    println!("{:?}", set);
//!    for v in set.into_iter() {
//!        println!("{:?}", v);
//!    }
//!}
//!
//! ```
pub mod check_fn;
mod impl_difference;
mod impl_entry;
mod impl_eq;
mod impl_serdes;
mod impl_set;
mod impl_sort;
pub mod derive {
    pub use mut_set_derive::item;
    pub use mut_set_derive::Dummy;
}
use core::{borrow::Borrow, hash::BuildHasher, ops::Deref};
pub use impl_entry::{Entry, OccupiedEntry, VacantEntry};
pub use impl_set::ValuesMut;
use std::{collections::HashMap, hash::RandomState, ops::DerefMut};
/// See more at [![github](https://img.shields.io/badge/github-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
/// ```
///#[derive(Debug)]
///#[mut_set::derive::item]
///pub struct MyItem<T1, T2>
///where
///    T1: Sized,
///{
///    #[id]
///    pub(self) id1: usize,
///    pub(crate) ctx1: T1,
///    pub ctx2: T2,
///    #[id]
///    pub id2: String,
///}
///#[test]
///fn test() {
///    let mut set = mut_set::MutSet::new();
///    println!("{:?}", set);
///    set.insert(MyItem {
///        id1: 2,
///        id2: "www".to_string(),
///        ctx1: -1,
///        ctx2: "ccc".to_string(),
///    });
///    set.insert(MyItem {
///        id1: 1,
///        id2: "ww".to_string(),
///        ctx1: -2,
///        ctx2: "cc".to_string(),
///    });
///    println!("{:?}", set);
///    for v in set.iter() {
///        println!("{:?}", v);
///    }
///    for v in set.iter_mut() {
///        v.ctx1 = 0;
///        println!("{:?}", v.id1);
///        // In `iter_mut` IDs write will be prohibited
///        // v.id1 = 0;
///    }
///    println!("{:?}", set);
///    println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string())));
///    set.replace(MyItem {
///        id1: 1,
///        id2: "ww".to_string(),
///        ctx1: -2,
///        ctx2: "cc".to_string(),
///    });
///    println!("{:?}", set);
///    for v in set.into_iter() {
///        println!("{:?}", v);
///    }
///}
/// ```
pub struct MutSet<T: Item, S: BuildHasher = RandomState> {
    hasher: S,
    inner: HashMap<u64, T, NoHashBuildHasher>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoHashBuildHasher;
impl NoHashBuildHasher {
    pub const fn new() -> Self {
        Self
    }
}
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

impl<T: Item, S: BuildHasher> MutSet<T, S> {
    #[inline]
    fn id(&self, item: &T) -> u64 {
        *item.id(&self).borrow()
    }
}

pub trait Item
where
    Self: Sized + MutSetDeref<Target = Self::ImmutIdItem>,
{
    type Id: Borrow<u64> + From<u64>;
    type ImmutIdItem: Deref<Target = Self> + From<Self> + Into<Self>;
    type MutSet<S: BuildHasher + Default>: Deref<Target = MutSet<Self, S>>
        + DerefMut
        + From<MutSet<Self, S>>
        + Into<MutSet<Self, S>>;
    fn id<S: BuildHasher>(&self, __set: &MutSet<Self, S>) -> Self::Id;
}

pub trait MutSetDeref {
    type Target;
    fn mut_set_deref(&mut self) -> &mut Self::Target;
}
