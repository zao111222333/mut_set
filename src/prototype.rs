#[derive(Debug)]
#[repr(C)]
// #[mut_set::Item(impl_hash)]
// #[mut_set::Item]
pub struct MyItem {
    // #[id]
    pub id1: usize,
    // #[id]
    pub id2: String,
    pub ctx1: isize,
    pub ctx2: String,
}
impl std::hash::Hash for MyItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id1.hash(state);
        self.id2.hash(state);
    }
}

#[doc(hidden)]
mod my_group_id_read_only{
    #[repr(C)]
    pub struct MyItemIdReadOnly {
        id1: usize,
        id2: String,
        pub ctx1: isize,
        pub ctx2: String,
    }

    impl crate::Item for super::MyItem {
        type ItemIdReadOnly = MyItemIdReadOnly;
    }
    impl core::ops::Deref for MyItemIdReadOnly {
        type Target = super::MyItem;
        fn deref(&self) -> &Self::Target {
            unsafe { &*(self as *const Self as *const Self::Target) }
        }
    }
    impl From<super::MyItem> for MyItemIdReadOnly {
        fn from(value: super::MyItem) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }
    impl From<MyItemIdReadOnly> for super::MyItem{
        fn from(value: MyItemIdReadOnly) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }
    impl std::fmt::Debug for MyItemIdReadOnly {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let _item: &super::MyItem = std::ops::Deref::deref(&self);
            _item.fmt(f)
        }
    }
}

#[test]
fn test(){
    let mut m = crate::MutSet::new();
    println!("{:?}",m);
    m.insert(MyItem { id1: 2, id2: "www".to_string(), ctx1: -1, ctx2: "ccc".to_string() });
    m.insert(MyItem { id1: 1, id2: "ww".to_string(), ctx1: -2, ctx2: "cc".to_string() });
    println!("{:?}",m);
    for v in m.iter(){
        println!("{}",v.id1);
    }
    for v in m.iter_mut(){
        v.ctx1 = 0;
        // In `iter_mut` IDs write will be prohibited
        // v.id1 = 0;
    }
    println!("{:?}",m);
}