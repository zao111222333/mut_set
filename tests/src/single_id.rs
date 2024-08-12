// cargo expand --manifest-path ./tests/Cargo.toml single_id

#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
#[mut_set::derive::item]
pub(super) struct MyItem<T1>
where
    T1: Sized,
{
    #[id]
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
}

#[test]
fn test() {
    let mut set = mut_set::MutSet::new();
    println!("{:?}", set);
    set.insert(MyItem { id1: 2, ctx1: -1 });
    set.insert(MyItem { id1: 1, ctx1: -2 });
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
    println!("{:?}", set.get(&2));
    set.replace(MyItem { id1: 1, ctx1: -2 });
    println!("{:?}", set);
    for v in set.into_iter() {
        println!("{:?}", v);
    }
}
