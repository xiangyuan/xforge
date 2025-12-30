//! # xforge-scheme
//!
//! Support for Xcode scheme files (.xcscheme).
//!
//! A scheme defines a collection of targets to build, a configuration to use
//! when building, and a collection of tests to execute.

mod scheme;
mod actions;
mod buildable_reference;

pub use scheme::Scheme;
pub use actions::*;
pub use buildable_reference::{BuildableReference, BlueprintIdentifier};

pub type Result<T> = std::result::Result<T, anyhow::Error>;
