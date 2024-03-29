# mut_set

Use the idea of [readonly](https://crates.io/crates/readonly) to implement HashSet with `iter_mut`.

Add crates by following command

``` shell
cargo add mut_set mut_set_derive
```

or add it into `Cargo.toml`

```toml
[dependencies]
mut_set = "0.2"
mut_set_derive = "0.2"
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

The macro will implement all stuffs in [tests/src/prototype.rs](tests/src/prototype.rs).

Take `Xxx` as an example:

+ Create two struct `ImmutIdXxx`, and `XxxId`. Where `ImmutIdXxx` is same to `Xxx` with private id fields, and `XxxId` only contains id fields.
+ Do rearrangement so that all id fields are located at beginning of the structure. By the help of `#[repr(C)]`, we can use raw pointer operations to (zero-cost?) convert `Xxx`, `ImmutIdXxx`, and `XxxId`.
+ `impl mut_set::Item for Xxx<ImmutIdItem = ImmutIdXxx>`
+ `MutSet<T: Item> = HashMap<u64, T::ImmutIdItem>`, where the `u64` is the hash value.
+ Wrap the iteration function
    + `iter(&self) -> Iter<&Xxx>`
    + `into_iter(self) -> Iter<Xxx>`
    + `iter_mut(&mut self) -> Iter<&mut ImmutIdXxx>`

## Other functions

+ If you want to add some `derive`/`proc_macro` to `ImmutIdXxx`, and `XxxId`. You can add arguments to `mut_set_derive::item`, and use `;` to separate. e.g.,

    ``` rust
    #[mut_set_derive::item(derive(Debug, Clone); derive(derivative::Derivative); derivative(Default);)]
    struct Xxx {}
    ```

    will impl

    ``` rust
    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    struct ImmutIdXxx {}

    #[derive(Debug, Clone)]
    #[derive(derivative::Derivative)]
    #[derivative(Default)]
    struct XxxId {}
    ```