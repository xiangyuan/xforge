//! Type-safe handle system

use crate::ObjectId;
use std::marker::PhantomData;
use std::fmt;

/// Type-safe handle for PBX objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    id: ObjectId,
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    /// Create a new handle from ObjectId
    pub fn from_id(id: ObjectId) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    /// Get the underlying ObjectId
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
}

impl<T> fmt::Display for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
