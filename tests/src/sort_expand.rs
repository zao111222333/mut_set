#![allow(clippy::all, unused)]

#[cfg(not(doc))]
#[derive(Debug, derivative::Derivative)]
#[derive(Clone)]
#[derivative(Default)]
#[repr(C)]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub id2: String,
    pub id3: (),
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    use super::*;
    use std::{
        borrow::Borrow,
        fmt::Debug,
        hash::{BuildHasher, Hash, Hasher},
        ops::{Deref, DerefMut},
    };
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        id1: usize,
        id2: String,
        id3: (),
        pub(crate) ctx1: T1,
        pub(in super::super) ctx2: T2,
    }
    #[allow(clippy::ref_option_ref)]
    impl<T1, T2> MyItem<T1, T2>
    where
        T1: Sized,
    {
        const CHECK: () = {
            fn id2(id: &String) -> &str {
                id.borrow()
            }
        };
        #[inline]
        pub(in super::super) fn new_id<S: BuildHasher>(
            __set: &mut_set::MutSet<MyItem<T1, T2>, S>,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> <MyItem<T1, T2> as mut_set::Item>::Id {
            let mut state = __set.hasher().build_hasher();
            Hash::hash(&id1, &mut state);
            Hash::hash(&id2, &mut state);
            Hash::hash(&id3, &mut state);
            MyItemId(state.finish())
        }
    }
    #[doc(hidden)]
    impl<T1, T2> PartialEq for MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id3 == other.id3 && self.id2 == other.id2 && self.id1 == other.id1
        }
    }
    #[doc(hidden)]
    impl<T1, T2> Eq for MyItem<T1, T2> where T1: Sized {}
    #[doc(hidden)]
    #[allow(clippy::non_canonical_partial_ord_impl)]
    impl<T1, T2> PartialOrd for MyItem<T1, T2>
    where
        T1: Sized,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.id1.partial_cmp(&other.id1) {
                Some(core::cmp::Ordering::Equal) | None => {}
                ord => return ord,
            }
            match self.id2.partial_cmp(&other.id2) {
                Some(core::cmp::Ordering::Equal) | None => {}
                ord => return ord,
            }
            self.id3.partial_cmp(&other.id3)
        }
    }
    #[doc(hidden)]
    impl<T1, T2> Ord for MyItem<T1, T2>
    where
        T1: Sized,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }
    pub(in super::super) struct MyItemId(u64);
    impl core::borrow::Borrow<u64> for MyItemId {
        #[inline]
        fn borrow(&self) -> &u64 {
            &self.0
        }
    }
    pub(in super::super) struct MutSetMyItem<S: BuildHasher, T1, T2>(
        mut_set::MutSet<MyItem<T1, T2>, S>,
    );

    impl<S: BuildHasher, T1, T2> MutSetMyItem<S, T1, T2> {
        #[inline]
        pub(in super::super) fn contains(
            &self,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> bool {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_contains(&id)
        }
        #[inline]
        pub(in super::super) fn get(
            &self,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> Option<&MyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_get(&id)
        }
        #[inline]
        pub(in super::super) fn get_mut(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> Option<&mut ImmutIdMyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_get_mut(&id)
        }
        #[inline]
        pub(in super::super) fn remove(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> bool {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_remove(&id)
        }
        #[inline]
        pub(in super::super) fn take(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &(),
        ) -> Option<MyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_take(&id)
        }
    }
    impl<S: BuildHasher, T1, T2> Deref for MutSetMyItem<S, T1, T2> {
        type Target = mut_set::MutSet<MyItem<T1, T2>, S>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<S: BuildHasher, T1, T2> DerefMut for MutSetMyItem<S, T1, T2> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<S: BuildHasher, T1, T2> From<mut_set::MutSet<MyItem<T1, T2>, S>>
        for MutSetMyItem<S, T1, T2>
    {
        #[inline]
        fn from(value: mut_set::MutSet<MyItem<T1, T2>, S>) -> Self {
            Self(value)
        }
    }
    impl<S: BuildHasher, T1, T2> From<MutSetMyItem<S, T1, T2>>
        for mut_set::MutSet<MyItem<T1, T2>, S>
    {
        #[inline]
        fn from(value: MutSetMyItem<S, T1, T2>) -> Self {
            value.0
        }
    }
    impl<S: BuildHasher, T1: Debug, T2: Debug> Debug for MutSetMyItem<S, T1, T2> {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl<T1, T2> mut_set::Item for MyItem<T1, T2>
    where
        T1: Sized,
    {
        type Id = MyItemId;
        type ImmutIdItem = ImmutIdMyItem<T1, T2>;
        type MutSet<S: BuildHasher + Default> = MutSetMyItem<S, T1, T2>;

        #[inline]
        fn id<S: BuildHasher>(&self, __set: &mut_set::MutSet<Self, S>) -> Self::Id {
            let mut state = __set.hasher().build_hasher();
            Hash::hash(&self.id1, &mut state);
            Hash::hash(&self.id2, &mut state);
            Hash::hash(&self.id3, &mut state);
            MyItemId(state.finish())
        }
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
