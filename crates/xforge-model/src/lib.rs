//! xforge-model - Domain models for XForge

pub mod platform;
pub mod project;
pub mod plist_manager;
pub mod entitlements;

pub use platform::{Platform, ProductType};
pub use project::{Project, ProjectMetadata};
pub use plist_manager::PlistManager;
pub use entitlements::EntitlementsManager;
