# mut_set

Use the idea of [readonly](https://crates.io/crates/readonly) to implement mutable hashset.

``` rust
#[mut_set::item]
pub struct MyItem {
    #[id]
    pub id1: usize,
    #[id]
    pub id2: String,
    pub ctx1: isize,
    pub ctx2: String,
}

let mut m = mut_set::MutSet::new();
    println!("{:?}",m);
    m.insert(MyItem { id1: 2, id2: "www".to_string(), ctx1: -1, ctx2: "ccc".to_string() });
    m.insert(MyItem { id1: 1, id2: "ww".to_string(), ctx1: -2, ctx2: "cc".to_string() });
```

## How can we do that

The macro will implement all staffs as what [tests/prototype.rs](tests/prototype.rs) dose.

## TODO

macro
