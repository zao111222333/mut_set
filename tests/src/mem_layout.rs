/// size = 80 (0x50), align = 0x8
struct RustLayout {
    id1: String,
    id2: String,
    id3: bool,
    id4: String,
    id5: bool,
}

/// size = 80 (0x50), align = 0x8
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

/// size = 88 (0x58), align = 0x8
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
    assert_eq!(80, std::mem::size_of::<WithLayout>());
    assert_eq!(88, std::mem::size_of::<WithoutLayout>());
}
