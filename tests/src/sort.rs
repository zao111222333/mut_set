extern crate mut_set;
// cargo expand --manifest-path ./tests/Cargo.toml sort

#[derive(Debug, derivative::Derivative, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "T1: serde::Serialize + serde::de::DeserializeOwned")]
#[derivative(Default)]
#[mut_set::derive::item(sort)]
pub(super) struct MyItem<T1>
where
    T1: Sized + Default + serde::Serialize + serde::de::DeserializeOwned,
{
    #[id]
    #[size = 8]
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
    #[id(borrow = "&str", with_ref = false)]
    #[size = 24]
    pub id2: String,
    #[id]
    #[size = 0]
    pub id3: (),
}

#[test]
fn test() {
    let mut set = mut_set::MutSet::new();
    println!("{:?}", set);
    set.insert(MyItem { id1: 2, id2: "www".to_string(), ctx1: -1, id3: () });
    let old_iterm = MyItem { id1: 1, id2: "ww".to_string(), ctx1: -2, id3: () };
    assert!(set.insert(old_iterm.clone()));
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
    println!("{:?}", set.id_get(&MyItem::new_id(&set, &2, "www", &())));
    let new_item = MyItem { id1: 1, id2: "ww".to_string(), ctx1: -2, id3: () };
    assert_eq!(set.id_get(&MyItem::new_id(&set, &1, &"ww", &())), Some(&old_iterm));
    assert_eq!(set.replace(new_item.clone()), Some(old_iterm));
    assert_eq!(set.id_get(&mut_set::Item::id(&new_item, &set)), Some(&new_item));
    println!("{:?}", set);
    for v in set.iter() {
        println!("{:?}", v);
    }
    println!("==================");
    set.sort();
    for v in set.into_iter() {
        println!("{:?}", v);
    }
}
