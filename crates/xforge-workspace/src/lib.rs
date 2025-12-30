//! # xforge-workspace
//!
//! Support for Xcode workspace files (.xcworkspace).
//!
//! A workspace is a container for multiple Xcode projects and other files,
//! allowing you to work on multiple projects in a single window.

mod workspace;
mod file_ref;
mod group;

pub use workspace::Workspace;
pub use file_ref::{FileRef, FileRefLocation};
pub use group::Group;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
