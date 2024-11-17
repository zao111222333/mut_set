use crate::{Item, MutSet};
use core::{fmt, hash::BuildHasher};
use serde::{
    de::{self, value::SeqDeserializer, IntoDeserializer, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{cmp, marker::PhantomData, mem};

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
        serializer.collect_seq(self.iter())
    }
}

impl<'de, T, S> Deserialize<'de> for MutSet<T, S>
where
    T: Deserialize<'de> + Item,
    S: BuildHasher + Default,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct VecVisitor<T, S> {
            marker1: PhantomData<T>,
            marker2: PhantomData<S>,
        }

        impl<'de, T: Deserialize<'de> + Item, S: BuildHasher + Default> Visitor<'de>
            for VecVisitor<T, S>
        where
            T: Deserialize<'de>,
        {
            type Value = MutSet<T, S>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                pub fn cautious<Element>(hint: Option<usize>) -> usize {
                    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

                    if mem::size_of::<Element>() == 0 {
                        0
                    } else {
                        cmp::min(
                            hint.unwrap_or(0),
                            MAX_PREALLOC_BYTES / mem::size_of::<Element>(),
                        )
                    }
                }
                let capacity = cautious::<T>(seq.size_hint());
                let mut values = MutSet::with_capacity_and_hasher(capacity, S::default());
                while let Some(value) = seq.next_element()? {
                    values.insert(value);
                }

                Ok(values)
            }
        }

        let visitor = VecVisitor { marker1: PhantomData, marker2: PhantomData };
        deserializer.deserialize_seq(visitor)
        // Ok(Vec::<T>::deserialize(deserializer)?.into())
    }
}
