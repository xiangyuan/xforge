//! # XForge
//!
//! A modern, type-safe Rust library for creating and manipulating Xcode project files.

pub use xforge_core::{Handle, ObjectId, Registry};
pub use xforge_core::error::{Error, Result};
pub use xforge_model::{Project, ProjectMetadata, Platform, ProductType};
pub use xforge_builder::ProjectBuilder;

/// Prelude module for convenient imports
pub mod prelude {
    pub use xforge_core::{Handle, ObjectId};
    pub use xforge_model::{Project, Platform, ProductType, ProjectMetadata};
    pub use xforge_builder::ProjectBuilder;
    pub use xforge_core::error::Result;
    pub use anyhow::anyhow;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_imports() {
        let _ = std::mem::size_of::<ObjectId>();
        let _ = std::mem::size_of::<Registry>();
    }
    
    #[test]
    fn test_project_builder() {
        let project = ProjectBuilder::new()
            .name("TestApp")
            .organization("TestOrg")
            .build();
        
        assert_eq!(project.name(), "TestApp");
        assert_eq!(project.metadata().organization.as_deref(), Some("TestOrg"));
    }
}
