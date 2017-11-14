use std::collections;
use std::hash::Hash;
use std::borrow::Borrow;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Map(Map<String, Value>),
}

/// Represents a git config key/value type.
#[derive(PartialEq, Clone, Debug)]
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}

type MapImpl<K, V> = collections::BTreeMap<K, V>;

impl Map<String, Value> {
    /// Makes a new empty Map.
    #[inline]
    pub fn new() -> Self {
        Map { map: MapImpl::new() }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        String: Borrow<Q>,
        Q: Ord + Eq + Hash,
    {
        self.map.get(key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[inline]
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    pub fn entry<S>(&mut self, key: S) -> Entry
    where
        S: Into<String>,
    {
        use std::collections::btree_map::Entry as EntryImpl;

        match self.map.entry(key.into()) {
            EntryImpl::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant: vacant }),
            EntryImpl::Occupied(occupied) => Entry::Occupied(OccupiedEntry { occupied: occupied }),
        }
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> collections::btree_map::IntoIter<K, V> {
        self.map.into_iter()
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
/// This enum is constructed from the [`entry`] method on [`Map`].
///
/// [`entry`]: struct.Map.html#method.entry
/// [`Map`]: struct.Map.html
pub enum Entry<'a> {
    /// A vacant Entry.
    Vacant(VacantEntry<'a>),
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a>),
}

/// A vacant Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a> {
    vacant: VacantEntryImpl<'a>,
}

/// An occupied Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
pub struct OccupiedEntry<'a> {
    occupied: OccupiedEntryImpl<'a>,
}

type VacantEntryImpl<'a> = collections::btree_map::VacantEntry<'a, String, Value>;
type OccupiedEntryImpl<'a> = collections::btree_map::OccupiedEntry<'a, String, Value>;

impl<'a> Entry<'a> {
    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &String {
        match *self {
            Entry::Vacant(ref e) => e.key(),
            Entry::Occupied(ref e) => e.key(),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
}

impl<'a> VacantEntry<'a> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    #[inline]
    pub fn key(&self) -> &String {
        self.vacant.key()
    }

    /// Sets the value of the entry with the VacantEntry's key, and returns a
    /// mutable reference to it.
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}

impl<'a> OccupiedEntry<'a> {
    /// Gets a reference to the key in the entry.
    #[inline]
    pub fn key(&self) -> &String {
        self.occupied.key()
    }

    /// Gets a reference to the value in the entry.
    #[inline]
    pub fn get(&self) -> &Value {
        self.occupied.get()
    }

    /// Gets a mutable reference to the value in the entry.
    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {
        self.occupied.get_mut()
    }

    /// Converts the entry into a mutable reference to its value.
    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }

    /// Sets the value of the entry with the `OccupiedEntry`'s key, and returns
    /// the entry's old value.
    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }

    /// Takes the value of the entry out of the map, and returns it.
    #[inline]
    pub fn remove(self) -> Value {
        self.occupied.remove()
    }
}
