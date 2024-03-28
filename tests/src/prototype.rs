#[derive(Debug)]
#[repr(C)]
// #[mut_set::item(impl_hash)]
// #[mut_set::item]
pub struct MyItem {
    // #[id]
    pub id1: usize,
    // #[id]
    pub id2: String,
    pub ctx1: isize,
    pub ctx2: String,
}

#[doc(hidden)]
mod my_item_immut_id{
    use std::{borrow::Borrow, hash::Hash};
    // ====== impl_hash ========
    #[derive(Hash)]
    #[repr(C)]
    pub struct MyItemId {
        pub id1: usize,
        pub id2: String,
    }

    impl super::MyItem {
        pub fn id(id1: usize,id2: String) -> MyItemId{
            MyItemId{ id1, id2 }
        }
    }
    
    impl std::borrow::Borrow<MyItemId> for super::MyItem {
        #[inline]
        fn borrow(&self) -> &MyItemId {
            unsafe { &*(self as *const Self as *const MyItemId) }
        }
    }

    impl std::hash::Hash for super::MyItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            <super::MyItem as Borrow<MyItemId>>::borrow(self).hash(state);
        }
    }
    // ====== impl_hash ========
    
    #[repr(C)]
    pub struct MyItemImmutId {
        id1: usize,
        id2: String,
        pub ctx1: isize,
        pub ctx2: String,
    }

    impl mut_set::Item for super::MyItem {
        type ItemImmutId = MyItemImmutId;
    }
    impl core::ops::Deref for MyItemImmutId {
        type Target = super::MyItem;
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }
    impl From<super::MyItem> for MyItemImmutId {
        fn from(value: super::MyItem) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }
    impl From<MyItemImmutId> for super::MyItem{
        fn from(value: MyItemImmutId) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }
    impl std::fmt::Debug for MyItemImmutId {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let _item: &super::MyItem = std::ops::Deref::deref(&self);
            _item.fmt(f)
        }
    }
}

#[test]
fn test(){
    let mut set = mut_set::MutSet::new();
    println!("{:?}",set);
    set.insert(MyItem { id1: 2, id2: "www".to_string(), ctx1: -1, ctx2: "ccc".to_string() });
    set.insert(MyItem { id1: 1, id2: "ww".to_string(), ctx1: -2, ctx2: "cc".to_string() });
    println!("{:?}",set);
    for v in set.iter(){
        println!("{:?}",v);
    }
    for v in set.iter_mut(){
        v.ctx1 = 0;
        println!("{:?}",v.id1);
        // In `iter_mut` IDs write will be prohibited
        // v.id1 = 0;
    }
    println!("{:?}",set);
    let id1 = MyItem::id(2, "www".to_string());
    println!("{:?}",set.get(&id1));
    for v in set.into_iter(){
        println!("{:?}",v);
    }
}