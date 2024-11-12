use crate::{Item, MutSet};
use core::hash::BuildHasher;
use serde::{
    de::{self, value::SeqDeserializer, IntoDeserializer},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<'de, T, S, E> IntoDeserializer<'de, E> for MutSet<T, S>
where
    T: IntoDeserializer<'de, E> + Item,
    S: BuildHasher,
    E: de::Error,
{
    type Deserializer = SeqDeserializer<<Self as IntoIterator>::IntoIter, E>;
    #[inline]
    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self.into_iter())
    }
}
impl<T, S> Serialize for MutSet<T, S>
where
    T: Serialize + Item,
    S: BuildHasher,
{
    #[inline]
    fn serialize<SS: Serializer>(&self, serializer: SS) -> Result<SS::Ok, SS::Error> {
        let v: Vec<&T> = self.iter().collect();
        v.serialize(serializer)
    }
}

impl<'de, T, S> Deserialize<'de> for MutSet<T, S>
where
    T: Deserialize<'de> + Item,
    S: BuildHasher + Default,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Vec::<T>::deserialize(deserializer)?.into())
    }
}
