//! Object registry for managing PBX objects

use crate::{Handle, ObjectId};
use std::collections::HashMap;

/// Registry for storing and managing PBX objects
pub struct Registry {
    objects: HashMap<ObjectId, Box<dyn std::any::Any>>,
}

impl Registry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    
    /// Get the number of objects in the registry
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    
    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
