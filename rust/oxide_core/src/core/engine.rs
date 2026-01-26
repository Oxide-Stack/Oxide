use std::any::{Any, TypeId};
use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::core::{Reducer, StoreHandle};

/// Registry for store handles keyed by reducer type.
///
/// This is useful when an application wants a single place to manage multiple
/// stores without passing individual handles around.
pub struct Engine {
    stores: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl Engine {
    /// Creates an empty engine.
    ///
    /// # Returns
    /// A new engine with no registered stores.
    pub fn new() -> Self {
        Self {
            stores: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a handle for the store associated with `R`, creating it if needed.
    ///
    /// If a store already exists for `R`, `initial_state` is ignored.
    ///
    /// # Returns
    /// A [`StoreHandle`] for the reducer `R`.
    pub async fn ensure_store<R: Reducer>(&self, initial_state: R::State) -> StoreHandle<R> {
        let type_id = TypeId::of::<R>();

        {
            let stores = self.stores.read().await;
            if let Some(existing) = stores.get(&type_id) {
                if let Some(handle) = existing.downcast_ref::<StoreHandle<R>>() {
                    return handle.clone();
                }
            }
        }

        // Double-check under the write lock to avoid racing with another creator.
        let mut stores = self.stores.write().await;
        if let Some(existing) = stores.get(&type_id) {
            if let Some(handle) = existing.downcast_ref::<StoreHandle<R>>() {
                return handle.clone();
            }
        }

        let handle = StoreHandle::<R>::new(initial_state);
        stores.insert(type_id, Box::new(handle.clone()));
        handle
    }

    /// Returns an existing store handle for `R`, if present.
    ///
    /// # Returns
    /// `Some(handle)` when a store exists for `R`, otherwise `None`.
    pub async fn get_store<R: Reducer>(&self) -> Option<StoreHandle<R>> {
        let stores = self.stores.read().await;
        stores
            .get(&TypeId::of::<R>())
            .and_then(|x| x.downcast_ref::<StoreHandle<R>>())
            .cloned()
    }

    /// Removes and returns the store handle for `R`, if present.
    ///
    /// # Returns
    /// The removed handle when present, otherwise `None`.
    pub async fn remove_store<R: Reducer>(&self) -> Option<StoreHandle<R>> {
        let mut stores = self.stores.write().await;
        let removed = stores.remove(&TypeId::of::<R>())?;
        removed
            .downcast::<StoreHandle<R>>()
            .ok()
            .map(|boxed| *boxed)
    }

    /// Removes all stores from the engine.
    pub async fn clear(&self) {
        let mut stores = self.stores.write().await;
        stores.clear();
    }
}

impl Default for Engine {
    /// Creates an empty engine.
    fn default() -> Self {
        Self::new()
    }
}
