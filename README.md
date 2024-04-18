# mut_set

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![github](https://img.shields.io/badge/repo-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
[![crates.io](https://shields.io/crates/v/mut_set.svg?style=flat-square&label=crates.io)](https://crates.io/crates/mut_set)
[![Docs](https://docs.rs/mut_set/badge.svg)](https://docs.rs/mut_set)

Use the idea of [readonly](https://crates.io/crates/readonly) to implement HashSet with `iter_mut` and `get_mut`.

Add crates by following command

``` shell
cargo add mut_set mut_set_derive
```

or add it into `Cargo.toml`

```toml
[dependencies]
mut_set = "0.3"
mut_set_derive = "0.3"
```

## Demo

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

## How does `mut_set` work

The macro will implement all stuffs in [tests/src/basic_expand.rs](tests/src/basic_expand.rs).

Take `Xxx` as an example:

+ Create two struct `ImmutIdXxx`, and `XxxId`. Where `ImmutIdXxx` is same to `Xxx` with private id fields, and `XxxId` only contains id fields.
+ Do rearrangement so that all id fields are located at beginning of the structure. By the help of `#[repr(C)]`, we can use raw pointer operations to (zero-cost?) convert `Xxx`, `ImmutIdXxx`, and `XxxId`.
+ `impl mut_set::Item for Xxx<ImmutIdItem = ImmutIdXxx>`
+ `MutSet<T: Item> = HashMap<u64, T::ImmutIdItem>`, where the `u64` is the hash value.
+ Wrap the iteration function
  + `iter(&self) -> Iter<&Xxx>`
  + `into_iter(self) -> Iter<Xxx>`
  + `iter_mut(&mut self) -> Iter<&mut ImmutIdXxx>`

## Other features

+ If you want to add some `derive`/`proc_macro` to `ImmutIdXxx`/`XxxId`. You can add arguments to `mut_set_derive::item`, to specify which `derive` should add to `ImmutIdXxx`/`XxxId`, and the filter for fileds attribute. e.g.,

    ``` rust
    #[mut_set_derive::item(
    macro(derive(Debug, Clone);
          derive(derivative::Derivative);
          derivative(Default);),
    attr_filter(derivative;)
    )]
    struct Xxx {
        #[id]
        id: usize,
        #[derivative(Default(value = "8"))]
        #[some_attr]
        ctx: usize,
    }
    ```

    will impl

    ``` rust
    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    struct ImmutIdXxx {
        id: usize,
        #[derivative(Default(value = "8"))]
        ctx: usize,
    }

    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    struct XxxId {
        id: usize,
    }
    ```

    Here `some_attr` is not in `attr_filter()`, so it will be removed. See more at [tests/src/derive.rs](tests/src/derive.rs) and [tests/src/derive_expand.rs](tests/src/derive_expand.rs).
