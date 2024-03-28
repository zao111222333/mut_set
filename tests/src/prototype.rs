#[derive(Debug)]
#[repr(C)]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    pub id1: usize,
    pub id2: String,
    pub ctx1: T1,
    pub(in crate::prototype) ctx2: T2,
}
#[doc(hidden)]
mod __my_item {
    use std::{
        borrow::Borrow,
        hash::{Hash, Hasher},
        ops::Deref,
    };
    #[derive(Debug)]
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct MyItemId<T1, T2>
    where
        T1: Sized,
    {
        pub id1: usize,
        pub id2: String,
        _p: std::marker::PhantomData<(T1, T2)>,
    }
    impl<T1, T2> Hash for MyItemId<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&self.id2, state);
            Hash::hash(&self.id1, state);
        }
    }
    #[derive(Debug)]
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct MyItemImmutId<T1, T2>
    where
        T1: Sized,
    {
        id1: usize,
        id2: String,
        pub ctx1: T1,
        pub(in crate::prototype) ctx2: T2,
    }
    impl<T1, T2> super::MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        pub(in super::super) fn id(id1: usize, id2: String) -> MyItemId<T1, T2> {
            MyItemId::<T1, T2> { _p: std::marker::PhantomData::<(T1, T2)>, id1, id2 }
        }
    }
    #[doc(hidden)]
    impl<T1, T2> Borrow<MyItemId<T1, T2>> for super::MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn borrow(&self) -> &MyItemId<T1, T2> {
            unsafe { &*(self as *const Self as *const MyItemId<T1, T2>) }
        }
    }
    impl<T1, T2> Hash for super::MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            <super::MyItem<T1, T2> as Borrow<MyItemId<T1, T2>>>::borrow(self).hash(state);
        }
    }
    impl<T1, T2> mut_set::Item for super::MyItem<T1, T2>
    where
        T1: Sized,
    {
        type ItemImmutId = MyItemImmutId<T1, T2>;
    }
    impl<T1, T2> Deref for MyItemImmutId<T1, T2>
    where
        T1: Sized,
    {
        type Target = super::MyItem<T1, T2>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }
    impl<T1, T2> From<super::MyItem<T1, T2>> for MyItemImmutId<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: super::MyItem<T1, T2>) -> Self {
            Self {
                ctx1: value.ctx1,
                ctx2: value.ctx2,
                id1: value.id1,
                id2: value.id2,
            }
        }
    }
    impl<T1, T2> From<MyItemImmutId<T1, T2>> for super::MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        fn from(value: MyItemImmutId<T1, T2>) -> Self {
            Self {
                ctx1: value.ctx1,
                ctx2: value.ctx2,
                id1: value.id1,
                id2: value.id2,
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
    });
    set.insert(MyItem {
        id1: 1,
        id2: "ww".to_string(),
        ctx1: -2,
        ctx2: "cc".to_string(),
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
    let id1 = MyItem::id(2, "www".to_string());
    println!("{:?}", set.get(&id1));
    for v in set.into_iter() {
        println!("{:?}", v);
    }
}
