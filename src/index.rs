use std::{
    cmp::Ord,
    collections::{BTreeMap, BTreeSet},
};
use typed_id::Id;

use crate::entity::Entity;

#[derive(Debug, Default)]
pub struct UniqueIndex<K, T>
where
    T: Entity<T>,
{
    map: BTreeMap<K, Id<T>>,
}

impl<K, T> UniqueIndex<K, T>
where
    T: Entity<T>,
    K: Ord,
{
    pub fn insert(&mut self, key: K, id: Id<T>) -> Option<Id<T>> {
        self.map.insert(key, id)
    }

    pub fn get(&self, key: &K) -> Option<Id<T>> {
        self.map.get(key).cloned()
    }

    pub fn has_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}

#[derive(Debug, Default)]
pub struct ReverseIndex<K, T>
where
    T: Entity<T>,
{
    map: BTreeMap<K, BTreeSet<Id<T>>>,
}

impl<K, T> ReverseIndex<K, T>
where
    T: Entity<T>,
    K: Ord,
{
    pub fn insert(&mut self, key: K, id: Id<T>) -> bool {
        if let Some(set) = self.map.get_mut(&key) {
            set.insert(id)
        } else {
            false
        }
    }

    pub fn get_ids(&self, key: &K) -> Option<Vec<Id<T>>> {
        let set = self.map.get(key)?;
        Some(set.iter().cloned().collect())
    }

    pub fn has_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
