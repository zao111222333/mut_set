use crate::MutSetDeref;

use super::{Item, MutSet};
use core::{borrow::Borrow, hash::BuildHasher, iter::Map};
use std::{
    collections::{
        hash_map::{IntoValues, Values},
        HashMap, HashSet, TryReserveError,
    },
    hash::RandomState,
};
impl<T: Item + Clone, S: BuildHasher + Clone> Clone for MutSet<T, S> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T: Item + std::fmt::Debug, S: BuildHasher> std::fmt::Debug for MutSet<T, S> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T: Item, S: Default + BuildHasher> Default for MutSet<T, S> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<T: Item> MutSet<T, RandomState> {
    #[inline]
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> MutSet<T, RandomState> {
        Self {
            inner: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }
}

impl<T, S> From<Vec<T>> for MutSet<T, S>
where
    T: Item,
    S: BuildHasher + Default,
{
    #[inline]
    fn from(item: Vec<T>) -> Self {
        item.into_iter().collect()
    }
}

impl<T, S> From<HashSet<T, S>> for MutSet<T, S>
where
    T: Item,
    S: BuildHasher + Default,
{
    #[inline]
    fn from(item: HashSet<T, S>) -> Self {
        item.into_iter().collect()
    }
}

impl<T, S> FromIterator<T> for MutSet<T, S>
where
    T: Item,
    S: BuildHasher + Default,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> MutSet<T, S> {
        let mut set = MutSet::with_hasher(Default::default());
        set.extend(iter);
        set
    }
}

impl<T, S, const N: usize> From<[T; N]> for MutSet<T, S>
where
    T: Item,
    S: BuildHasher + Default,
{
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let set1 = HashSet::from([1, 2, 3, 4]);
    /// let set2: HashSet<_> = [1, 2, 3, 4].into();
    /// assert_eq!(set1, set2);
    /// ```
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T, S> Extend<T> for MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|item| _ = self.insert(item));
    }
}

impl<T, S> MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the `HashSet`. The collection may reserve more space to speculatively
    /// avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let mut set: HashSet<i32> = HashSet::new();
    /// set.reserve(10);
    /// assert!(set.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the `HashSet`. The collection may reserve more space to speculatively
    /// avoid frequent reallocations. After calling `try_reserve`,
    /// capacity will be greater than or equal to `self.len() + additional` if
    /// it returns `Ok(())`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let mut set: HashSet<i32> = HashSet::new();
    /// set.try_reserve(10).expect("why is the test harness OOMing on a handful of bytes?");
    /// ```
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Shrinks the capacity of the set as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::with_capacity(100);
    /// set.insert(1);
    /// set.insert(2);
    /// assert!(set.capacity() >= 100);
    /// set.shrink_to_fit();
    /// assert!(set.capacity() >= 2);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Shrinks the capacity of the set with a lower limit. It will drop
    /// down no lower than the supplied limit while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::with_capacity(100);
    /// set.insert(1);
    /// set.insert(2);
    /// assert!(set.capacity() >= 100);
    /// set.shrink_to(10);
    /// assert!(set.capacity() >= 10);
    /// set.shrink_to(0);
    /// assert!(set.capacity() >= 2);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity)
    }
    #[inline]
    pub fn id_contains(&self, id: &T::Id) -> bool {
        self.inner.contains_key(id.borrow())
    }
    #[inline]
    pub fn id_get(&self, id: &T::Id) -> Option<&T> {
        self.inner.get(id.borrow())
    }
    #[inline]
    pub fn id_get_mut(&mut self, id: &T::Id) -> Option<&mut <T as Item>::ImmutIdItem> {
        self.inner.get_mut(id.borrow()).map(MutSetDeref::mut_set_deref)
    }

    // /// Inserts the given `item` into the set if it is not present, then
    // /// returns a reference to the item in the set.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(hash_set_entry)]
    // ///
    // /// use std::collections::HashSet;
    // ///
    // /// let mut set = HashSet::from([1, 2, 3]);
    // /// assert_eq!(set.len(), 3);
    // /// assert_eq!(set.get_or_insert(2), &2);
    // /// assert_eq!(set.get_or_insert(100), &100);
    // /// assert_eq!(set.len(), 4); // 100 was inserted
    // /// ```
    // #[inline]
    // // #[unstable(feature = "hash_raw_entry", issue = "56167")]
    // pub fn get_or_insert(&mut self, item: T) -> &T {
    //     // Although the raw entry gives us `&mut T`, we only return `&T` to be consistent with
    //     // `get`. Key mutation is "raw" because you're not supposed to affect `Eq` or `Hash`.
    //     let key = self.hash_one(&item);
    //     self.inner
    //         .raw_entry_mut()
    //         .from_key(&key)
    //         .or_insert(key, item.into())
    //         .1
    // }

    // /// Inserts an owned copy of the given `item` into the set if it is not
    // /// present, then returns a reference to the item in the set.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(hash_set_entry)]
    // ///
    // /// use std::collections::HashSet;
    // ///
    // /// let mut set: HashSet<String> = ["cat", "dog", "horse"]
    // ///     .iter().map(|&pet| pet.to_owned()).collect();
    // ///
    // /// assert_eq!(set.len(), 3);
    // /// for &pet in &["cat", "dog", "fish"] {
    // ///     let item = set.get_or_insert_owned(pet);
    // ///     assert_eq!(item, pet);
    // /// }
    // /// assert_eq!(set.len(), 4); // a new "fish" was inserted
    // /// ```
    // #[inline]
    // pub fn get_or_insert_owned<Q: ?Sized>(&mut self, item: &Q) -> &T
    // where
    //     T: Borrow<Q>,
    //     Q: Hash + ToOwned<Owned = T>,
    // {
    //     // Although the raw entry gives us `&mut T`, we only return `&T` to be consistent with
    //     // `get`. Key mutation is "raw" because you're not supposed to affect `Eq` or `Hash`.
    //     let key = self.hash_one(&item);
    //     // let v_ = item.to_owned().into();
    //     match self.inner.get(&key){
    //         Some(v) => return &v,
    //         None => todo!(),
    //     }
    //     match self.inner.insert(key,v_){
    //         Some(_) => todo!(),
    //         None => todo!(),
    //     }
    //     // match self.get(item){
    //     //     Some(t) => return t,
    //     //     None => self.insert(item.to_owned()).expect("www").deref(),
    //     // }
    //     // self.inner.get_or_insert_owned(item)
    // }

    // /// Inserts a item computed from `f` into the set if the given `item` is
    // /// not present, then returns a reference to the item in the set.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(hash_set_entry)]
    // ///
    // /// use std::collections::HashSet;
    // ///
    // /// let mut set: HashSet<String> = ["cat", "dog", "horse"]
    // ///     .iter().map(|&pet| pet.to_owned()).collect();
    // ///
    // /// assert_eq!(set.len(), 3);
    // /// for &pet in &["cat", "dog", "fish"] {
    // ///     let item = set.get_or_insert_with(pet, str::to_owned);
    // ///     assert_eq!(item, pet);
    // /// }
    // /// assert_eq!(set.len(), 4); // a new "fish" was inserted
    // /// ```
    // #[inline]
    // pub fn get_or_insert_with<Q: ?Sized, F>(&mut self, item: &Q, f: F) -> &T
    // where
    //     T: Borrow<Q>,
    //     Q: Hash + Eq,
    //     F: FnOnce(&Q) -> T,
    // {
    //     // Although the raw entry gives us `&mut T`, we only return `&T` to be consistent with
    //     // `get`. Key mutation is "raw" because you're not supposed to affect `Eq` or `Hash`.
    //     self.inner.get_or_insert_with(item, f)
    // }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let a = HashSet::from([1, 2, 3]);
    /// let mut b = HashSet::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint(&self, other: &MutSet<T, S>) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|item| !other.inner.contains_key(&other.id(item)))
        } else {
            other.iter().all(|item| !self.inner.contains_key(&self.id(item)))
        }
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the items in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let sup = HashSet::from([1, 2, 3]);
    /// let mut set = HashSet::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset(&self, other: &MutSet<T, S>) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|item| other.inner.contains_key(&other.id(item)))
        } else {
            false
        }
    }

    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the items in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let sub = HashSet::from([1, 2]);
    /// let mut set = HashSet::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    #[inline]
    pub fn is_superset(&self, other: &MutSet<T, S>) -> bool {
        other.is_subset(self)
    }

    /// Adds a item to the set.
    ///
    /// Returns whether the item was newly inserted. That is:
    ///
    /// - If the set did not previously contain this item, `true` is returned.
    /// - If the set already contained this item, `false` is returned,
    ///   and the set is not modified: original item is not replaced,
    ///   and the item passed as argument is dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.inner.entry(self.id(&item))
        {
            e.insert(item);
            true
        } else {
            false
        }
    }

    /// Adds a item to the set, replacing the existing item, if any, that is equal to the given
    /// one. Returns the replaced item.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::new();
    /// set.insert(Vec::<i32>::new());
    ///
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    /// set.replace(Vec::with_capacity(10));
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    /// ```
    #[inline]
    pub fn replace(&mut self, item: T) -> Option<T> {
        self.inner.insert(self.id(&item), item)
    }

    #[inline]
    pub fn id_remove(&mut self, id: &T::Id) -> bool {
        self.inner.remove(id.borrow()).is_some()
    }
    #[inline]
    pub fn id_take(&mut self, id: &T::Id) -> Option<T> {
        self.inner.remove(id.borrow())
    }
    #[inline]
    pub fn id_entry<DefaultF: FnOnce() -> T>(
        &mut self,
        id: &T::Id,
        default_f: DefaultF,
    ) -> crate::Entry<'_, T, DefaultF> {
        match (&mut self.inner).entry(*id.borrow()) {
            std::collections::hash_map::Entry::Occupied(inner) => {
                crate::Entry::Occupied(crate::OccupiedEntry::new(inner))
            }
            std::collections::hash_map::Entry::Vacant(inner) => {
                crate::Entry::Vacant(crate::VacantEntry::new(inner), default_f)
            }
        }
    }
}

impl<T, S> IntoIterator for MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = T;

    type IntoIter = IntoValues<u64, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_values()
    }
}

impl<'a, T, S> IntoIterator for &'a MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = &'a T;
    type IntoIter = Values<'a, u64, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.values()
    }
}

impl<'a, T, S> IntoIterator for &'a mut MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = &'a mut <T as MutSetDeref>::Target;
    type IntoIter = ValuesMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.values_mut().map(MutSetDeref::mut_set_deref)
    }
}

pub type ValuesMut<'a, T> = Map<
    std::collections::hash_map::ValuesMut<'a, u64, T>,
    fn(&mut T) -> &mut <T as MutSetDeref>::Target,
>;
impl<T, S> MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    #[inline]
    pub fn iter_mut(&mut self) -> ValuesMut<'_, T> {
        self.into_iter()
    }
    /// Returns the number of elements the set can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let set: HashSet<i32> = HashSet::with_capacity(100);
    /// assert!(set.capacity() >= 100);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let mut set = HashSet::new();
    /// set.insert("a");
    /// set.insert("b");
    ///
    /// // Will print in an arbitrary order.
    /// for x in set.iter() {
    ///     println!("{x}");
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over set takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    #[inline]
    pub fn iter(&self) -> Values<'_, u64, T> {
        self.into_iter()
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut v = HashSet::new();
    /// assert_eq!(v.len(), 0);
    /// v.insert(1);
    /// assert_eq!(v.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut v = HashSet::new();
    /// assert!(v.is_empty());
    /// v.insert(1);
    /// assert!(!v.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // /// Clears the set, returning all elements as an iterator. Keeps the
    // /// allocated memory for reuse.
    // ///
    // /// If the returned iterator is dropped before being fully consumed, it
    // /// drops the remaining elements. The returned iterator keeps a mutable
    // /// borrow on the set to optimize its implementation.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use std::collections::HashSet;
    // ///
    // /// let mut set = HashSet::from([1, 2, 3]);
    // /// assert!(!set.is_empty());
    // ///
    // /// // print 1, 2, 3 in an arbitrary order
    // /// for i in set.drain() {
    // ///     println!("{i}");
    // /// }
    // ///
    // /// assert!(set.is_empty());
    // /// ```
    // #[inline]
    // pub fn drain(&mut self) -> std::collections::hash_map::Drain<'_, u64, T> {
    //     // Drain { base: self.base.drain() }
    //     self.inner.drain().into_iter().collect()
    // }

    //     /// Creates an iterator which uses a closure to determine if a item should be removed.
    //     ///
    //     /// If the closure returns true, then the item is removed and yielded.
    //     /// If the closure returns false, the item will remain in the list and will not be yielded
    //     /// by the iterator.
    //     ///
    //     /// If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating
    //     /// or the iteration short-circuits, then the remaining elements will be retained.
    //     /// Use [`retain`] with a negated predicate if you do not need the returned iterator.
    //     ///
    //     /// [`retain`]: HashSet::retain
    //     ///
    //     /// # Examples
    //     ///
    //     /// Splitting a set into even and odd items, reusing the original set:
    //     ///
    //     /// ```
    //     /// #![feature(hash_extract_if)]
    //     /// use std::collections::HashSet;
    //     ///
    //     /// let mut set: HashSet<i32> = (0..8).collect();
    //     /// let extracted: HashSet<i32> = set.extract_if(|v| v % 2 == 0).collect();
    //     ///
    //     /// let mut evens = extracted.into_iter().collect::<Vec<_>>();
    //     /// let mut odds = set.into_iter().collect::<Vec<_>>();
    //     /// evens.sort();
    //     /// odds.sort();
    //     ///
    //     /// assert_eq!(evens, vec![0, 2, 4, 6]);
    //     /// assert_eq!(odds, vec![1, 3, 5, 7]);
    //     /// ```
    //     #[inline]
    //     pub fn extract_if<F>(&mut self, pred: F) -> std::collections::hash_set::ExtractIf<'_, T, F>
    //     where
    //         F: FnMut(&T) -> bool,
    //     {
    //         // let mut f_mut = f;
    //         self.inner.(|k:&u64, v:&mut T::ImmutIdItem|
    //             f_mut(Deref::deref(&*v))
    //         )
    // // pred
    //         // // let x= self.inner.extract_if(|f|{});
    //         // todo!()
    //         // std::collections::hash_set::ExtractIf { base: self.inner.extract_if(pred) }
    //     }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::from([1, 2, 3, 4, 5, 6]);
    /// set.retain(|&k| k % 2 == 0);
    /// assert_eq!(set, HashSet::from([2, 4, 6]));
    /// ```
    ///
    /// # Performance
    ///
    /// In the current implementation, this operation takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut f_mut = f;
        self.inner.retain(|_, item| f_mut(&*item))
    }

    /// Clears the set, removing all items.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// let mut v = HashSet::new();
    /// v.insert(1);
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Creates a new empty hash set which will use the given hasher to hash
    /// keys.
    ///
    /// The hash set is also created with the default initial capacity.
    ///
    /// Warning: `hasher` is normally randomly generated, and
    /// is designed to allow `HashSet`s to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::hash::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut set = HashSet::with_hasher(s);
    /// set.insert(2);
    /// ```
    #[inline]
    // #[stable(feature = "hashmap_build_hasher", since = "1.7.0")]
    // #[rustc_const_unstable(feature = "const_collections_with_hasher", issue = "102575")]
    pub fn with_hasher(hasher: S) -> MutSet<T, S> {
        MutSet { inner: HashMap::with_hasher(hasher) }
    }

    /// Creates an empty `HashSet` with at least the specified capacity, using
    /// `hasher` to hash the keys.
    ///
    /// The hash set will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, the hash set will not allocate.
    ///
    /// Warning: `hasher` is normally randomly generated, and
    /// is designed to allow `HashSet`s to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::hash::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut set = HashSet::with_capacity_and_hasher(10, s);
    /// set.insert(1);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> MutSet<T, S> {
        MutSet {
            inner: HashMap::with_capacity_and_hasher(capacity, hasher),
        }
    }

    /// Returns a reference to the set's [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::hash::RandomState;
    ///
    /// let hasher = RandomState::new();
    /// let set: HashSet<i32> = HashSet::with_hasher(hasher);
    /// let hasher: &RandomState = set.hasher();
    /// ```
    #[inline]
    pub fn hasher(&self) -> &S {
        self.inner.hasher()
    }
}
