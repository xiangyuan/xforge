//! # xforge-config
//!
//! Support for Xcode configuration files (.xcconfig).
//!
//! An xcconfig file contains build settings in a simple text format,
//! supporting conditional settings, include directives, and variable expansion.

mod config;
mod parser;
mod conditional;

pub use config::XCConfig;
pub use conditional::{BuildContext, ConditionalSetting};

pub type Result<T> = std::result::Result<T, anyhow::Error>;
