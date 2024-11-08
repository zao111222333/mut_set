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
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    use super::*;
    use std::{
        borrow::Borrow,
        hash::{BuildHasher, Hash, Hasher},
        ops::Deref,
    };
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        id1: usize,
        id2: String,
        id3: Option<String>,
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
            fn id3(id: &Option<String>) -> Option<&str> {
                id.as_ref().map(core::borrow::Borrow::borrow)
            }
        };
        #[inline]
        pub(in super::super) fn new_id<S: BuildHasher>(
            __set: &mut_set::MutSet<MyItem<T1, T2>, S>,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> <MyItem<T1, T2> as mut_set::Item>::Id {
            let mut state = __set.hasher().build_hasher();
            Hash::hash(&id1, &mut state);
            Hash::hash(&id2, &mut state);
            Hash::hash(&id3, &mut state);
            MyItemId(state.finish())
        }
    }
    pub(in super::super) struct MyItemId(u64);
    impl core::borrow::Borrow<u64> for MyItemId {
        #[inline]
        fn borrow(&self) -> &u64 {
            &self.0
        }
    }
    impl<T1, T2> mut_set::Item for MyItem<T1, T2>
    where
        T1: Sized,
    {
        type Id = MyItemId;
        type ImmutIdItem = ImmutIdMyItem<T1, T2>;
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
