// cargo expand --manifest-path ./tests/Cargo.toml basic
#[inline]
const fn f64_into_hash_ord_fn(val: &f64) -> ordered_float::OrderedFloat<f64> {
    ordered_float::OrderedFloat(*val)
}

#[derive(Debug)]
#[mut_set::derive::item]
pub(super) struct MyItem<T1> {
    #[id]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
    #[id(into_hash_ord_fn = f64_into_hash_ord_fn)]
    pub id2: f64,
    #[id]
    pub id3: Option<String>,
}

#[test]
fn test() {
    use mut_set::MutSetExt;

    let mut set = indexmap::IndexSet::new();
    set.insert(MyItem { id1: 2, id2: 4.2, ctx1: -1, id3: None });
    set.insert(MyItem { id1: 1, id2: 3.2, ctx1: -2, id3: None });
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
    println!("{:?}", set.get(&MyItemId::new(2, 4.2, None)));
    set.replace(MyItem { id1: 1, id2: 3.2, ctx1: -2, id3: None });
    println!("{:?}", set);
    for v in set.into_iter() {
        println!("{:?}", v);
    }
}
