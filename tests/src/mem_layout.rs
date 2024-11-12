#[expect(dead_code)]
struct RustLayout {
    id1: String,
    id2: String,
    id3: bool,
    id4: String,
    id5: bool,
}
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[mut_set::derive::item]
struct WithLayout {
    #[id]
    #[size = 24]
    id1: String,
    #[id]
    #[size = 24]
    id2: String,
    #[id]
    #[size = 1]
    id3: bool,
    #[id]
    #[size = 24]
    id4: String,
    #[id]
    #[size = 1]
    id5: bool,
}

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[mut_set::derive::item]
struct WithoutLayout {
    #[id]
    id1: String,
    #[id]
    id2: String,
    #[id]
    id3: bool,
    #[id]
    id4: String,
    #[id]
    id5: bool,
}

#[test]
fn assert_size() {
    assert_eq!(80, std::mem::size_of::<RustLayout>());
    assert_eq!(88, std::mem::size_of::<WithoutLayout>());
    assert_eq!(88, std::mem::size_of::<__without_layout::ImmutIdWithoutLayout>());
    let with_layout_size: usize;
    cfg_if::cfg_if! {
        if #[cfg(feature = "__dbg_disable_mem_layout")] {
            with_layout_size = 88;
        } else {
            with_layout_size = 80;
        }
    };
    assert_eq!(with_layout_size, std::mem::size_of::<WithLayout>());
    assert_eq!(with_layout_size, std::mem::size_of::<__with_layout::ImmutIdWithLayout>());
}
