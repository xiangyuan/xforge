//! xforge-core - Core types and utilities for XForge
//!
//! This crate provides the foundational types used throughout XForge:
//! - `ObjectId`: Collision-resistant UUID generation
//! - `Handle<T>`: Type-safe object references
//! - `Registry`: Object storage and management
//! - `PBXObject`: Common trait for all PBX objects

pub mod error;
pub mod handle;
pub mod object_id;
pub mod registry;
pub mod traits;

pub mod product_type;

pub use error::{Error, Result};
pub use handle::Handle;
pub use object_id::ObjectId;
pub use registry::Registry;
pub use traits::PBXObject;
pub use product_type::ProductType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_id_generation() {
        let id1 = ObjectId::generate();
        let id2 = ObjectId::generate();
        
        // IDs should be different
        assert_ne!(id1, id2);
        
        // UUID strings should be 24 characters
        let uuid1 = id1.to_uuid_string();
        assert_eq!(uuid1.len(), 24);
        assert!(uuid1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(uuid1.chars().all(|c| !c.is_ascii_lowercase()));
    }
    
    #[test]
    fn test_object_id_roundtrip() {
        let id = ObjectId::generate();
        let uuid_string = id.to_uuid_string();
        let parsed = ObjectId::from_uuid_string(&uuid_string).unwrap();
        
        assert_eq!(id, parsed);
    }
    
    #[test]
    fn test_registry_creation() {
        let registry = Registry::new();
        assert_eq!(registry.len(), 0);
    }
}
