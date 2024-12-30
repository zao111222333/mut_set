// cargo expand --manifest-path ./tests/Cargo.toml basic

#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[mut_set::derive::item]
#[serde(bound = "T1: serde::Serialize + serde::de::DeserializeOwned")]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized + Default,
    T2: Sized + Default,
{
    #[id]
    #[size = 8]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
    #[id(borrow = "&str", with_ref = false)]
    #[size = 24]
    pub id2: String,
    #[id(
        borrow = "Option<&str>",
        check_fn = "mut_set::borrow_option!",
        with_ref = false
    )]
    #[size = 24]
    pub id3: Option<String>,
}

#[test]
fn test() {
    use mut_set::Item;
    use std::ops::Deref;

    let mut set = <MyItem<i32, String> as Item>::MutSet::<std::hash::RandomState>::from(
        mut_set::MutSet::new(),
    );
    set.insert(MyItem {
        id1: 2,
        id2: "www".to_string(),
        ctx1: -1,
        ctx2: "ccc".to_string(),
        id3: None,
    });
    set.insert(MyItem {
        id1: 1,
        id2: "ww".to_string(),
        ctx1: -2,
        ctx2: "cc".to_string(),
        id3: None,
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
    println!("{:?}", set.deref());
    println!("{:?}", set.get(&2, "www", None));
    set.replace(MyItem {
        id1: 1,
        id2: "ww".to_string(),
        ctx1: -2,
        ctx2: "cc".to_string(),
        id3: None,
    });
    // let a = Some("".to_string());
    // let b: Option<&str> = a.as_ref().map(|s| s.borrow());
    // let s = "".to_string();
    // let b: &str = s.borrow();
    println!("{:?}", set.deref());
    for v in set.into_iter() {
        println!("{:?}", v);
    }
}
