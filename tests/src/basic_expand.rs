#![allow(clippy::all, unused)]

#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
#[repr(C)]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub id2: String,
    pub id3: Option<String>,
    pub ctx1: T1,
    pub(super) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    use super::*;
    use std::{
        borrow::Borrow,
        hash::{Hash, Hasher},
        ops::{Deref, DerefMut},
    };
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct MyItemId<T1, T2>
    where
        T1: Sized,
    {
        pub(super) id1: usize,
        pub id2: String,
        pub id3: Option<String>,
        _p: std::marker::PhantomData<(T1, T2)>,
    }
    impl<T1, T2> Hash for MyItemId<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&self.id1, state);
            Hash::hash(&self.id2, state);
            Hash::hash(&self.id3, state);
        }
    }
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        id1: usize,
        id2: String,
        pub id3: Option<String>,
        pub(crate) ctx1: T1,
        pub(in super::super) ctx2: T2,
    }
    impl<T1, T2> MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        pub(in super::super) fn new_id(
            id1: usize,
            id2: String,
            id3: Option<String>,
        ) -> MyItemId<T1, T2> {
            MyItemId::<T1, T2> {
                _p: std::marker::PhantomData::<(T1, T2)>,
                id1,
                id2,
                id3,
            }
        }
        #[inline]
        pub(in super::super) fn id(&self) -> &MyItemId<T1, T2> {
            <MyItem<T1, T2> as Borrow<MyItemId<T1, T2>>>::borrow(self)
        }
        #[allow(noop_method_call)]
        const CHECK: () = {
            fn id2(id: &String) -> &str {
                id.borrow()
            }
            fn id3(id: &Option<String>) -> Option<&str> {
                id.as_ref().map(core::borrow::Borrow::borrow)
            }
        };
        #[inline]
        pub(in super::super) fn borrow_id(
            __set: &mut_set::MutSet<MyItem<T1, T2>>,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> <MyItem<T1, T2> as mut_set::Item>::BorrowId {
            use std::hash::BuildHasher;
            let mut state = __set.hasher().build_hasher();
            Hash::hash(id1, &mut state);
            Hash::hash(id2, &mut state);
            Hash::hash(id3, &mut state);
            state.finish().into()
        }
    }
    #[doc(hidden)]
    impl<T1, T2> Borrow<MyItemId<T1, T2>> for MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn borrow(&self) -> &MyItemId<T1, T2> {
            unsafe { &*(self as *const Self as *const MyItemId<T1, T2>) }
        }
    }
    impl<T1, T2> Hash for MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            <MyItem<T1, T2> as Borrow<MyItemId<T1, T2>>>::borrow(self).hash(state);
        }
    }
    pub(in super::super) struct MyItemBorrowId(u64);
    impl From<u64> for MyItemBorrowId {
        #[inline]
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl Into<u64> for MyItemBorrowId {
        #[inline]
        fn into(self) -> u64 {
            self.0
        }
    }
    impl<T1, T2> mut_set::Item for MyItem<T1, T2>
    where
        T1: Sized,
    {
        type Id = MyItemId<T1, T2>;
        type BorrowId = MyItemBorrowId;
        type ImmutIdItem = ImmutIdMyItem<T1, T2>;
    }
    impl<T1, T2> Deref for ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        type Target = MyItem<T1, T2>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }
    impl<T1, T2> mut_set::MutSetDeref for MyItem<T1, T2>
    where
        T1: Sized,
    {
        type Target = ImmutIdMyItem<T1, T2>;
        #[inline]
        fn mut_set_deref(&mut self) -> &mut Self::Target {
            unsafe { &mut *(self as *mut Self as *mut Self::Target) }
        }
    }
}

// #[test]
// fn test() {
//     let mut set = mut_set::MutSet::new();
//     println!("{:?}", set);
//     set.insert(MyItem {
//         id1: 2,
//         id2: "www".to_string(),
//         ctx1: -1,
//         ctx2: "ccc".to_string(),
//     });
//     set.insert(MyItem {
//         id1: 1,
//         id2: "ww".to_string(),
//         ctx1: -2,
//         ctx2: "cc".to_string(),
//     });
//     println!("{:?}", set);
//     for v in set.iter() {
//         println!("{:?}", v);
//     }
//     for v in set.iter_mut() {
//         v.ctx1 = 0;
//         println!("{:?}", v.id1);
//         // In `iter_mut` IDs write will be prohibited
//         // v.id1 = 0;
//     }
//     println!("{:?}", set);
//     println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string())));
//     set.replace(MyItem {
//         id1: 1,
//         id2: "ww".to_string(),
//         ctx1: -2,
//         ctx2: "cc".to_string(),
//     });
//     set.get_borrow(MyItem::borrow_id(&set, &1, "ww"));
//     println!("{:?}", set);
//     for v in set.into_iter() {
//         println!("{:?}", v);
//     }
// }
