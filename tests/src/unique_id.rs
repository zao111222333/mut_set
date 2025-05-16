// cargo expand --manifest-path ./tests/Cargo.toml unique_id
#[derive(Debug)]
#[mut_set::derive::item]
pub(super) struct MyItem {
    #[id(borrow = str)]
    pub(self) id: String,
    pub ctx: f64,
}

#[test]
fn test() {
    use mut_set::MutSetExt;

    let mut set = indexmap::IndexSet::new();
    set.insert(MyItem { id: "1".into(), ctx: 1.0 });
    set.insert(MyItem { id: "2".into(), ctx: 2.0 });
    println!("{:?}", set);
    for v in set.iter() {
        println!("{:?}", v);
    }
    for v in set.iter_mut() {
        v.ctx = 0.0;
        // In `iter_mut` IDs write will be prohibited
        // v.id = "0".into();
    }
    println!("{:?}", set);
    println!("{:?}", set.get("1"));
}
