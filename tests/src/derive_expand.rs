#[cfg(not(doc))]
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
    pub id3: (),
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    use super::*;
    use std::{
        borrow::Borrow,
        hash::{Hash, Hasher},
        ops::Deref,
    };
    #[doc(hidden)]
    #[derive(Debug, derivative::Derivative)]
    #[derive(Clone)]
    #[derivative(Default)]
    #[repr(C)]
    pub(in super::super) struct MyItemId<T1, T2>
    where
        T1: Sized,
    {
        #[derivative(Default(value = "8"))]
        pub(super) id1: usize,
        pub id2: String,
        pub id3: (),
        _p: std::marker::PhantomData<(T1, T2)>,
    }
    impl<T1, T2> Hash for MyItemId<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&self.id3, state);
            Hash::hash(&self.id2, state);
            Hash::hash(&self.id1, state);
        }
    }
    #[doc(hidden)]
    #[derive(Debug, derivative::Derivative)]
    #[derive(Clone)]
    #[derivative(Default)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        #[derivative(Default(value = "8"))]
        id1: usize,
        id2: String,
        id3: (),
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
            id3: (),
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
            self.borrow()
        }
    }
    #[doc(hidden)]
    impl<T1, T2> PartialEq for MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id3.eq(&other.id3) && self.id2.eq(&other.id2) && self.id1.eq(&other.id1)
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
            match self.id1.cmp(&other.id1) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
            match self.id2.cmp(&other.id2) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
            self.id3.cmp(&other.id3)
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
    impl<T1, T2> mut_set::Item for MyItem<T1, T2>
    where
        T1: Sized,
    {
        type ImmutIdItem = ImmutIdMyItem<T1, T2>;
        type Id = MyItemId<T1, T2>;
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
    impl<T1, T2> From<MyItem<T1, T2>> for ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: MyItem<T1, T2>) -> Self {
            Self {
                ctx1: value.ctx1,
                ctx2: value.ctx2,
                id1: value.id1,
                id2: value.id2,
                id3: value.id3,
            }
        }
    }
    impl<T1, T2> From<ImmutIdMyItem<T1, T2>> for MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: ImmutIdMyItem<T1, T2>) -> Self {
            Self {
                ctx1: value.ctx1,
                ctx2: value.ctx2,
                id1: value.id1,
                id2: value.id2,
                id3: value.id3,
            }
        }
    }
}
#[test]
fn test() {
    let mut set = mut_set::MutSet::new();
    println!("{:?}", set);
    set.insert(MyItem {
        id1: 2,
        id2: "www".to_string(),
        ctx1: -1,
        ctx2: "ccc".to_string(),
        id3: (),
    });
    set.insert(MyItem {
        id1: 1,
        id2: "ww".to_string(),
        ctx1: -2,
        ctx2: "cc".to_string(),
        id3: (),
    });
    println!("{:?}", set);
    for v in set.iter() {
        println!("{:?}", v);
    }
    for v in set.iter_mut() {
        v.ctx1 = 0;
        println!("{:?}", v.id1);
        // In `iter_mut` IDs write will be prohibited
        // v.id1 = 0;
    }
    println!("{:?}", set);
    println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string(), ())));
    set.replace(MyItem {
        id1: 1,
        id2: "ww".to_string(),
        ctx1: -2,
        ctx2: "cc".to_string(),
        id3: (),
    });
    println!("{:?}", set);
    for v in set.into_iter_sort() {
        println!("{:?}", v);
    }
}
