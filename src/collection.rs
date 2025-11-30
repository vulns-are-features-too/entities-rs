use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use typed_id::Id;

use crate::entity::Entity;

#[derive(Debug, Default)]
pub struct EntityCollection<T>
where
    T: Entity<T>,
{
    map: BTreeMap<Id<T>, T>,
}

impl<T> EntityCollection<T>
where
    T: Entity<T>,
{
    pub fn insert(&mut self, e: T) -> Option<T> {
        self.map.insert(e.id(), e)
    }

    pub fn get(&self, id: &Id<T>) -> Option<&T> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: &Id<T>) -> Option<&mut T> {
        self.map.get_mut(id)
    }

    pub fn has_key(&self, id: &Id<T>) -> bool {
        self.map.contains_key(id)
    }
}

type EntityRef<T> = Rc<RefCell<T>>;

#[derive(Debug, Default)]
pub struct SharedEntityCollection<T>
where
    T: Entity<T>,
{
    map: BTreeMap<Id<T>, EntityRef<T>>,
}

impl<T> SharedEntityCollection<T>
where
    T: Entity<T>,
{
    pub fn insert(&mut self, e: T) -> Result<EntityRef<T>, T> {
        let id = e.id();
        if self.map.contains_key(&id) {
            return Err(e);
        }

        let entity_ref = Rc::new(RefCell::new(e));
        match self.map.insert(id, entity_ref.clone()) {
            None => Ok(entity_ref.clone()),
            Some(x) => Err(Rc::try_unwrap(x)
                .expect("Failed to get value back from Rc")
                .into_inner()),
        }
    }

    pub fn get(&self, id: &Id<T>) -> Option<EntityRef<T>> {
        Some(self.map.get(id)?.clone())
    }

    pub fn has_key(&self, id: &Id<T>) -> bool {
        self.map.contains_key(id)
    }
}
