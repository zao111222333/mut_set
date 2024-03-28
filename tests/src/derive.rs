// cargo expand --manifest-path ./tests/Cargo.toml derive

#[derive(Debug)]
#[mut_set_derive::item]
pub(super) struct MyItem<TT: Sized> {
    #[id]
    pub id1: usize,
    pub ctx1: TT,
    pub(in crate::derive) ctx2: String,
    #[id]
    pub id2: String,
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
