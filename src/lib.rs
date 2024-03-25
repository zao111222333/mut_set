mod prototype;

use std::{collections::HashMap, hash::Hash};

pub trait Item where
    Self: Hash+Sized,
{
    type ItemIdReadOnly: From<Self> + Into<Self> + core::ops::Deref<Target = Self>;
    #[inline]
    fn hash_value(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        std::hash::Hasher::finish(&hasher)
    }
}

pub struct MutSet<T:Item>{
    inner: HashMap<u64,T::ItemIdReadOnly>
}

impl<T:Item+std::fmt::Debug> std::fmt::Debug for MutSet<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T:Item>  MutSet<T>{
    #[inline]
    pub fn new()->Self{
        Self { inner: HashMap::new() }
    }
    #[inline]
    pub fn insert(&mut self, v: T) -> Option<<T as Item>::ItemIdReadOnly> {
        let hash_value = <T as Item>::hash_value(&v);
        let read_only_v: <T as Item>::ItemIdReadOnly = v.into();
        self.inner.insert(hash_value, read_only_v)
    }
    
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T>{
        self.inner.iter().map(|(_,v)| <<T as Item>::ItemIdReadOnly as core::ops::Deref>::deref(&v) ).into_iter()
    }
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T>{
        self.inner.into_iter().map(|(_,v)| <<T as Item>::ItemIdReadOnly as Into::<T>>::into(v) ).into_iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut <T as Item>::ItemIdReadOnly>{
        self.inner.iter_mut().map(|(_,v)|v).into_iter()
    }
}

// impl core::ops::Deref for GroupSet {
//     type Target = HashMap<u64,MyGroupIdReadOnly>;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }


// use std::{collections::HashMap, hash::Hash};
// #[cfg(not(doc))]
// #[repr(C)]
// pub struct MyGroup {
//     id1: usize,
//     pub id2: String,
//     pub ctx1: isize,
//     pub ctx2: String,
// }
// #[automatically_derived]
// impl ::core::default::Default for MyGroup {
//     #[inline]
//     fn default() -> MyGroup {
//         MyGroup {
//             id1: ::core::default::Default::default(),
//             id2: ::core::default::Default::default(),
//             ctx1: ::core::default::Default::default(),
//             ctx2: ::core::default::Default::default(),
//         }
//     }
// }
// const _: () = {
//     #[doc(hidden)]
//     #[repr(C)]
//     pub struct ReadOnlyMyGroup {
//         pub id1: usize,
//         pub id2: String,
//         pub ctx1: isize,
//         pub ctx2: String,
//     }
//     #[automatically_derived]
//     impl ::core::default::Default for ReadOnlyMyGroup {
//         #[inline]
//         fn default() -> ReadOnlyMyGroup {
//             ReadOnlyMyGroup {
//                 id1: ::core::default::Default::default(),
//                 id2: ::core::default::Default::default(),
//                 ctx1: ::core::default::Default::default(),
//                 ctx2: ::core::default::Default::default(),
//             }
//         }
//     }
//     #[doc(hidden)]
//     impl core::ops::Deref for MyGroup {
//         type Target = ReadOnlyMyGroup;
//         fn deref(&self) -> &Self::Target {
//             unsafe { &*(self as *const Self as *const Self::Target) }
//         }
//     }
// };

