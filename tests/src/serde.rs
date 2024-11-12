// cargo expand --manifest-path ./tests/Cargo.toml serde

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Debug, derivative::Derivative, Eq, PartialEq)]
#[derivative(Default)]
#[mut_set::derive::item]
pub(super) struct MyItem<T1, T2>
where
    T1: Sized,
{
    #[id]
    #[derivative(Default(value = "8"))]
    pub(self) id1: usize,
    pub(crate) ctx1: T1,
    pub(super) ctx2: T2,
    #[id]
    pub id2: String,
}

#[test]
fn test() {
    let set = mut_set::MutSet::<MyItem<i32, String>>::from(vec![
        MyItem {
            id1: 1,
            id2: "ww".to_string(),
            ctx1: -2,
            ctx2: "cc".to_string(),
        },
        MyItem {
            id1: 2,
            id2: "ww".to_string(),
            ctx1: -3,
            ctx2: "cc".to_string(),
        },
    ]);
    println!("origin = {:?}", set);
    let serialized = serde_json::to_string(&set).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: mut_set::MutSet<MyItem<i32, String>> =
        serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    assert_eq!(set, deserialized);
    assert_ne!(set, mut_set::MutSet::<MyItem<i32, String>>::new());
}
