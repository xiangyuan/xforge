//! Object registry for managing PBX objects

use crate::{Handle, ObjectId, PBXObject};
use std::collections::HashMap;

/// Registry for storing and managing PBX objects
pub struct Registry {
    objects: HashMap<String, Box<dyn PBXObject>>,
}

impl Registry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    
    /// Register an object and return a handle
    pub fn register<T: PBXObject + 'static>(&mut self, object: T) -> Handle<T> {
        let id = ObjectId::generate();
        let handle = Handle::from_id(id);
        self.objects.insert(id.to_uuid_string(), Box::new(object));
        handle
    }
    
    /// Get the number of objects in the registry
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    
    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
    
    /// Iterate over all objects
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Box<dyn PBXObject>)> {
        self.objects.iter()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
