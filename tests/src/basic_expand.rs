#![allow(clippy::all, unused)]

#[derive(Debug)]
#[repr(C)]
pub(super) struct MyItem<T1> {
    pub(self) id1: usize,
    pub id2: String,
    pub id3: Option<String>,
    pub(crate) ctx1: T1,
}
#[doc(hidden)]
#[repr(C)]
pub(super) struct MyItemId {
    pub(self) id1: usize,
    pub id2: f64,
    pub id3: Option<String>,
}
impl MyItemId {
    #[inline]
    pub fn new(id1: usize, id2: f64, id3: Option<String>) -> Self {
        Self { id1, id2, id3 }
    }
}
#[doc(hidden)]
#[expect(clippy::field_scoped_visibility_modifiers)]
mod __my_item {
    #[expect(clippy::wildcard_imports)]
    use super::*;
    use core::{
        borrow::Borrow,
        hash::{Hash, Hasher},
        ops::Deref,
    };
    use mut_set::Item as _;
    #[doc(hidden)]
    #[repr(C)]
    pub(in super::super) struct IdReadonlyMyItem<T1> {
        id1: usize,
        id2: f64,
        id3: Option<String>,
        pub(crate) ctx1: T1,
    }
    #[doc(hidden)]
    impl Hash for MyItemId {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            Hash::hash(&self.id1, state);
            Hash::hash(&f64_into_hash_ord_fn(&self.id2), state);
            Hash::hash(&self.id3, state);
        }
    }
    #[doc(hidden)]
    impl PartialEq for MyItemId {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id1 == other.id1
                && f64_into_hash_ord_fn(&self.id2) == f64_into_hash_ord_fn(&other.id2)
                && self.id3 == other.id3
        }
    }
    #[doc(hidden)]
    impl Eq for MyItemId {}
    #[doc(hidden)]
    #[allow(clippy::non_canonical_partial_ord_impl)]
    impl PartialOrd for MyItemId {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.id1.partial_cmp(&other.id1) {
                Some(core::cmp::Ordering::Equal) | None => {}
                ord => return ord,
            }
            match f64_into_hash_ord_fn(&self.id2)
                .partial_cmp(&f64_into_hash_ord_fn(&other.id2))
            {
                Some(core::cmp::Ordering::Equal) | None => {}
                ord => return ord,
            }
            self.id3.partial_cmp(&other.id3)
        }
    }
    #[doc(hidden)]
    impl Ord for MyItemId {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }
    impl<T1> Borrow<MyItemId> for MyItem<T1> {
        fn borrow(&self) -> &MyItemId {
            unsafe { &*(self as *const Self as *const MyItemId) }
        }
    }
    impl<T1> Borrow<MyItemId> for IdReadonlyMyItem<T1> {
        fn borrow(&self) -> &MyItemId {
            unsafe { &*(self as *const Self as *const MyItemId) }
        }
    }
    #[doc(hidden)]
    impl<T1> Hash for MyItem<T1> {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id().hash(state)
        }
    }
    #[doc(hidden)]
    impl<T1> PartialEq for MyItem<T1> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id().eq(other.id())
        }
    }
    #[doc(hidden)]
    impl<T1> Eq for MyItem<T1> {}
    #[doc(hidden)]
    #[allow(clippy::non_canonical_partial_ord_impl)]
    impl<T1> PartialOrd for MyItem<T1> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.id().partial_cmp(other.id())
        }
    }
    #[doc(hidden)]
    impl<T1> Ord for MyItem<T1> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.id().cmp(other.id())
        }
    }
    impl<T1> mut_set::Item for MyItem<T1> {
        type Id = MyItemId;
        type IdReadonlyItem = IdReadonlyMyItem<T1>;
        #[expect(invalid_reference_casting)]
        unsafe fn __unsafe_deref_mut(&self) -> &mut Self::IdReadonlyItem {
            unsafe { &mut *(self as *const Self as *mut Self::IdReadonlyItem) }
        }
    }
    impl<T1> Deref for IdReadonlyMyItem<T1> {
        type Target = MyItem<T1>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }
}
#[inline]
const fn f64_into_hash_ord_fn(val: &f64) -> ordered_float::OrderedFloat<f64> {
    ordered_float::OrderedFloat(*val)
}
