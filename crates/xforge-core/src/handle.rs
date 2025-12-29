//! Type-safe handle system

use crate::ObjectId;
use std::marker::PhantomData;

/// Type-safe handle for PBX objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    id: ObjectId,
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    /// Create a new handle
    pub(crate) fn new(id: ObjectId) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    /// Get the underlying ObjectId
    pub fn id(&self) -> ObjectId {
        self.id
    }
}
