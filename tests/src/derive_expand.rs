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
    pub ctx1: T1,
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
    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    #[repr(C)]
    pub(in super::super) struct MyItemId<T1, T2>
    where
        T1: Sized,
    {
        #[derivative(Default(value = "8"))]
        pub(super) id1: usize,
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
    #[doc(hidden)]
    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    #[repr(C)]
    pub(in super::super) struct ImmutIdMyItem<T1, T2>
    where
        T1: Sized,
    {
        #[derivative(Default(value = "8"))]
        id1: usize,
        id2: String,
        pub(crate) ctx1: T1,
        pub(in super::super) ctx2: T2,
    }
    impl<T1, T2> MyItem<T1, T2>
    where
        T1: Sized,
    {
        #[inline]
        pub(in super::super) fn new_id(id1: usize, id2: String) -> MyItemId<T1, T2> {
            MyItemId::<T1, T2> { _p: std::marker::PhantomData::<(T1, T2)>, id1, id2 }
        }
        #[inline]
        pub(in super::super) fn id(&self) -> &MyItemId<T1, T2> {
            self.borrow()
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
            }
        }
    }
}
