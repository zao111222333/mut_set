# mut_set

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![github](https://img.shields.io/badge/repo-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
[![crates.io](https://shields.io/crates/v/mut_set.svg?style=flat-square&label=crates.io)](https://crates.io/crates/mut_set)
[![Docs](https://docs.rs/mut_set/badge.svg)](https://docs.rs/mut_set)

Use the idea of [readonly](https://crates.io/crates/readonly) to implement `HashSet` & `IndexMap` with `iter_mut` and `get_mut`.

## Demo

``` rust
use mut_set::MutSetExt;

#[inline]
const fn f64_into_hash_ord_fn(val: &f64) -> ordered_float::OrderedFloat<f64> {
    ordered_float::OrderedFloat(*val)
}

#[derive(Debug, Default, Clone)]
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
```