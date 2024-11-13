use crate::Item;

pub enum Entry<'a, T: Item, DefaultF: FnOnce() -> T> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, T>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, T>, DefaultF),
}

pub struct OccupiedEntry<'a, T: Item>(
    std::collections::hash_map::OccupiedEntry<'a, u64, T>,
);
pub struct VacantEntry<'a, T: Item>(std::collections::hash_map::VacantEntry<'a, u64, T>);

impl<'a, T: 'a + Item> OccupiedEntry<'a, T> {
    #[inline]
    pub(crate) fn new(
        inner: std::collections::hash_map::OccupiedEntry<'a, u64, T>,
    ) -> Self {
        Self(inner)
    }
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> T::Id {
        (*self.0.key()).into()
    }

    /// Take the ownership of the key and value from the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     // We delete the entry from the map.
    ///     o.remove_entry();
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    pub fn remove_entry(self) -> (T::Id, T) {
        let (id, item) = self.0.remove_entry();
        (id.into(), item)
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.get(), &12);
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        self.0.get()
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: Self::into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     *o.get_mut() += 10;
    ///     assert_eq!(*o.get(), 22);
    ///
    ///     // We can use the same Entry multiple times.
    ///     *o.get_mut() += 2;
    /// }
    ///
    /// assert_eq!(map["poneyland"], 24);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut T::ImmutIdItem {
        self.0.get_mut().mut_set_deref()
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: Self::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// assert_eq!(map["poneyland"], 12);
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     *o.into_mut() += 10;
    /// }
    ///
    /// assert_eq!(map["poneyland"], 22);
    /// ```
    #[inline]
    pub fn into_mut(self) -> &'a mut T::ImmutIdItem {
        self.0.into_mut().mut_set_deref()
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(mut o) = map.entry("poneyland") {
    ///     assert_eq!(o.insert(15), 12);
    /// }
    ///
    /// assert_eq!(map["poneyland"], 15);
    /// ```
    #[inline]
    pub fn insert(&mut self, value: T) -> T {
        self.0.insert(value)
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// map.entry("poneyland").or_insert(12);
    ///
    /// if let Entry::Occupied(o) = map.entry("poneyland") {
    ///     assert_eq!(o.remove(), 12);
    /// }
    ///
    /// assert_eq!(map.contains_key("poneyland"), false);
    /// ```
    #[inline]
    pub fn remove(self) -> T {
        self.0.remove()
    }

    /// Replaces the entry, returning the old key and value. The new key in the hash map will be
    /// the key used to create this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(map_entry_replace)]
    /// use std::collections::hash_map::{Entry, HashMap};
    /// use std::rc::Rc;
    ///
    /// let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    /// map.insert(Rc::new("Stringthing".to_string()), 15);
    ///
    /// let my_key = Rc::new("Stringthing".to_string());
    ///
    /// if let Entry::Occupied(entry) = map.entry(my_key) {
    ///     // Also replace the key with a handle to our other key.
    ///     let (old_key, old_value): (Rc<String>, u32) = entry.replace_entry(16);
    /// }
    ///
    /// ```
    #[inline]
    #[cfg(feature = "map_entry_replace")]
    pub fn replace_entry(self, value: T) -> (T::Id, T) {
        let (id, item) = self.0.replace_entry(value);
        (id.into(), item)
    }

    /// Replaces the key in the hash map with the key used to create this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(map_entry_replace)]
    /// use std::collections::hash_map::{Entry, HashMap};
    /// use std::rc::Rc;
    ///
    /// let mut map: HashMap<Rc<String>, u32> = HashMap::new();
    /// let known_strings: Vec<Rc<String>> = Vec::new();
    ///
    /// // Initialise known strings, run program, etc.
    ///
    /// reclaim_memory(&mut map, &known_strings);
    ///
    /// fn reclaim_memory(map: &mut HashMap<Rc<String>, u32>, known_strings: &[Rc<String>] ) {
    ///     for s in known_strings {
    ///         if let Entry::Occupied(entry) = map.entry(Rc::clone(s)) {
    ///             // Replaces the entry's key with our version of it in `known_strings`.
    ///             entry.replace_key();
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    #[cfg(feature = "map_entry_replace")]
    pub fn replace_key(self) -> T::Id {
        self.0.replace_key().into()
    }
}

impl<'a, T: 'a + Item> VacantEntry<'a, T> {
    #[inline]
    pub(crate) fn new(
        inner: std::collections::hash_map::VacantEntry<'a, u64, T>,
    ) -> Self {
        Self(inner)
    }
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> T::Id {
        (*self.0.key()).into()
    }

    /// Take ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("poneyland") {
    ///     v.into_key();
    /// }
    /// ```
    #[inline]
    pub fn into_key(self) -> T::Id {
        self.0.into_key().into()
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[inline]
    pub fn insert(self, value: T) -> &'a mut T::ImmutIdItem {
        self.0.insert(value).mut_set_deref()
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns an `OccupiedEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(entry_insert)]
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::Entry;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// if let Entry::Vacant(o) = map.entry("poneyland") {
    ///     o.insert_entry(37);
    /// }
    /// assert_eq!(map["poneyland"], 37);
    /// ```
    #[inline]
    #[cfg(feature = "entry_insert")]
    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        let base = self.0.insert_entry(value);
        OccupiedEntry(base)
    }
}

impl<'a, T: Item, DefaultF: FnOnce() -> T> Entry<'a, T, DefaultF> {
    // /// Ensures a value is in the entry by inserting the default if empty, and returns
    // /// a mutable reference to the value in the entry.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use std::collections::HashMap;
    // ///
    // /// let mut map: HashMap<&str, u32> = HashMap::new();
    // ///
    // /// map.entry("poneyland").or_insert(3);
    // /// assert_eq!(map["poneyland"], 3);
    // ///
    // /// *map.entry("poneyland").or_insert(10) *= 2;
    // /// assert_eq!(map["poneyland"], 6);
    // /// ```
    // #[inline]
    // pub fn or_insert(self, default: T) -> &'a mut T::ImmutIdItem {
    //     match self {
    //         Entry::Occupied(entry) => entry.into_mut(),
    //         Entry::Vacant(entry, _) => entry.insert(default.into()),
    //     }
    // }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// let value = "hoho";
    ///
    /// map.entry("poneyland").or_insert_with(|| value);
    ///
    /// assert_eq!(map["poneyland"], "hoho");
    /// ```
    #[inline]
    pub fn or_insert_with<F: FnOnce(&mut T::ImmutIdItem)>(
        self,
        default: F,
    ) -> &'a mut T::ImmutIdItem {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry, default_f) => {
                let mut item: T::ImmutIdItem = default_f().into();
                default(&mut item);
                entry.insert(item.into())
            }
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, usize> = HashMap::new();
    ///
    /// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    ///
    /// assert_eq!(map["poneyland"], 9);
    /// ```
    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&T::Id) -> T>(
        self,
        default: F,
    ) -> &'a mut T::ImmutIdItem {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry, _) => {
                let value = default(&entry.key());
                entry.insert(value.into())
            }
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    /// assert_eq!(map.entry("poneyland").key(), &"poneyland");
    /// ```
    #[inline]
    pub fn key(&self) -> T::Id {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry, _) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, u32> = HashMap::new();
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 42);
    ///
    /// map.entry("poneyland")
    ///    .and_modify(|e| { *e += 1 })
    ///    .or_insert(42);
    /// assert_eq!(map["poneyland"], 43);
    /// ```
    #[inline]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut T::ImmutIdItem),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry, default_f) => Entry::Vacant(entry, default_f),
        }
    }

    /// Sets the value of the entry, and returns an `OccupiedEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(entry_insert)]
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<&str, String> = HashMap::new();
    /// let entry = map.entry("poneyland").insert_entry("hoho".to_string());
    ///
    /// assert_eq!(entry.key(), &"poneyland");
    /// ```
    #[inline]
    #[cfg(feature = "entry_insert")]
    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }
    #[inline]
    // #[stable(feature = "entry_or_default", since = "1.28.0")]
    pub fn or_default(self) -> &'a mut T::ImmutIdItem {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry, default_f) => entry.insert(default_f()),
        }
    }
}

// impl<'a, T: Item + Default> Entry<'a, T> {
//     /// Ensures a value is in the entry by inserting the default value if empty,
//     /// and returns a mutable reference to the value in the entry.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # fn main() {
//     /// use std::collections::HashMap;
//     ///
//     /// let mut map: HashMap<&str, Option<u32>> = HashMap::new();
//     /// map.entry("poneyland").or_default();
//     ///
//     /// assert_eq!(map["poneyland"], None);
//     /// # }
//     /// ```
//     #[inline]
//     // #[stable(feature = "entry_or_default", since = "1.28.0")]
//     pub fn or_default(self) -> &'a mut T::ImmutIdItem {
//         match self {
//             Entry::Occupied(entry) => entry.into_mut(),
//             Entry::Vacant(entry) => entry.insert(Default::default()),
//         }
//     }
// }
