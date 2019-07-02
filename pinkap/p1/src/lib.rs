#![deny(missing_docs)]

//! Crate that create a CLI for  Key / Value Store :

use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Create a KvStore
    pub fn new() -> KvStore {
        let map = HashMap::new();

        KvStore { map }
    }
    ///Sets a Key value pair inside a KvStore
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    ///Gets a Key value pair inside a KvStore
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Option<String> {
        match self.map.get(&key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    ///Removes a Key value pair inside a KvStore
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
