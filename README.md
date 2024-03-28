# mut_set

Use the idea of [readonly](https://crates.io/crates/readonly) to implement hashset with `iter_mut`.

Add crates by following command

``` shell
cargo add mut_set mut_set_derive
```

``` rust
#[derive(Debug)]
#[mut_set_derive::item]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    #[id]
    pub id1: usize,
    pub ctx1: T1,
    pub(in crate::derive) ctx2: T2,
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
```

## How can we do that

The macro will implement all staffs as what [tests/src/prototype.rs](tests/src/prototype.rs) dose.
