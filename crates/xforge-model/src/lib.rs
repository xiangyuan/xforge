//! xforge-model - Domain models for XForge

pub mod platform;
pub mod project;

pub use platform::{Platform, ProductType};
pub use project::{Project, ProjectMetadata};
