mod set;
use std::{
    collections::HashMap,
    hash::{Hash, RandomState},
    ops::Deref,
};

pub trait Item
where
    Self: Hash + Sized,
{
    type ItemImmutId: From<Self> + Into<Self> + Deref<Target = Self>;
}

pub struct MutSet<T: Item, S = RandomState> {
    inner: HashMap<u64, T::ItemImmutId, S>,
}
