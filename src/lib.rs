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
mod impl_difference;
mod impl_eq;
mod impl_serdes;
mod impl_set;
mod impl_sort;
pub mod derive {
    pub use mut_set_derive::item;
}
use core::{
    hash::{BuildHasher, Hash},
    ops::Deref,
};
use std::{collections::HashMap, hash::RandomState};

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
    inner: HashMap<u64, T, S>,
}

pub trait Item
where
    Self: Hash + Sized + MutSetDeref<Target = Self::ImmutIdItem>,
{
    type ImmutIdItem: Deref<Target = Self>;
    type Id: Hash + ?Sized;
}

pub trait MutSetDeref {
    type Target;
    fn mut_set_deref(&mut self) -> &mut Self::Target;
}
