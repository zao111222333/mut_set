#![allow(clippy::all, unused)]

#[derive(Debug, derivative::Derivative, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derivative(Default)]
#[repr(C)]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    pub id2: String,
    pub id3: Option<String>,
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    #[expect(clippy::wildcard_imports)]
    use super::*;
    use core::{
        borrow::Borrow,
        hash::{BuildHasher, Hash, Hasher},
        ops::{Deref, DerefMut},
    };
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        id2: String,
        id3: Option<String>,
        id1: usize,
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
    // #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, Default)]
    pub(in super::super) struct MutSetMyItem<S: BuildHasher + Default, T1, T2>(
        mut_set::MutSet<MyItem<T1, T2>, S>,
    );
    impl<S: BuildHasher + Default, T1, T2> MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        pub(in super::super) fn contains(
            &self,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> bool {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_contains(&id)
        }
        #[inline]
        pub(in super::super) fn get(
            &self,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> Option<&MyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_get(&id)
        }
        #[inline]
        pub(in super::super) fn get_mut(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> Option<&mut ImmutIdMyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_get_mut(&id)
        }
        #[inline]
        pub(in super::super) fn remove(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> bool {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_remove(&id)
        }
        #[inline]
        pub(in super::super) fn take(
            &mut self,
            id1: &usize,
            id2: &str,
            id3: &Option<&str>,
        ) -> Option<MyItem<T1, T2>> {
            let id = MyItem::new_id(&self, id1, id2, id3);
            self.id_take(&id)
        }
    }
    impl<S: BuildHasher + Default, T1, T2> Deref for MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        type Target = mut_set::MutSet<MyItem<T1, T2>, S>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<S: BuildHasher + Default, T1, T2> DerefMut for MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<S: BuildHasher + Default, T1, T2> From<mut_set::MutSet<MyItem<T1, T2>, S>>
        for MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: mut_set::MutSet<MyItem<T1, T2>, S>) -> Self {
            Self(value)
        }
    }
    impl<S: BuildHasher + Default, T1, T2> From<MutSetMyItem<S, T1, T2>>
        for mut_set::MutSet<MyItem<T1, T2>, S>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: MutSetMyItem<S, T1, T2>) -> Self {
            value.0
        }
    }
    impl<S: BuildHasher + Default, T1, T2> IntoIterator for MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        type Item = MyItem<T1, T2>;
        type IntoIter = std::collections::hash_map::IntoValues<u64, MyItem<T1, T2>>;
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    impl<'a, S: BuildHasher + Default, T1, T2> IntoIterator for &'a MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        type Item = &'a MyItem<T1, T2>;
        type IntoIter = std::collections::hash_map::Values<'a, u64, MyItem<T1, T2>>;
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            (&self.0).into_iter()
        }
    }
    impl<'a, S: BuildHasher + Default, T1, T2> IntoIterator
        for &'a mut MutSetMyItem<S, T1, T2>
    where
        T1: Sized,
    {
        type Item = &'a mut ImmutIdMyItem<T1, T2>;
        type IntoIter = mut_set::ValuesMut<'a, MyItem<T1, T2>>;
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            (&mut self.0).into_iter()
        }
    }
    use serde::{
        de::{self, value::SeqDeserializer, IntoDeserializer},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    // impl<'de, T1, T2, S, E> IntoDeserializer<'de, E>
    //     for <MyItem<T1, T2> as mut_set::Item>::MutSet<S>
    // where
    //     T1: IntoDeserializer<'de, E>,
    //     T2: IntoDeserializer<'de, E>,
    //     S: BuildHasher,
    //     E: de::Error,
    // {
    //     type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;
    //     #[inline]
    //     fn into_deserializer(self) -> Self::Deserializer {
    //         SeqDeserializer::new(self.into_iter())
    //     }
    // }
    // impl<S, T1, T2> Serialize for <MyItem<T1, T2> as mut_set::Item>::MutSet<S>
    // where
    //     T1: Serialize,
    //     T2: Serialize,
    //     S: BuildHasher + Default,
    // {
    //     #[inline]
    //     fn serialize<SS: Serializer>(&self, serializer: SS) -> Result<SS::Ok, SS::Error> {
    //         let v: Vec<&MyItem<T1, T2>> = self.iter().collect();
    //         v.serialize(serializer)
    //     }
    // }

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
