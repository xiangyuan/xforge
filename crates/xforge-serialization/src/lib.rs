//! xforge-serialization - Xcode project serialization

pub mod plist_writer;
pub mod plist_parser;

pub use plist_writer::{PlistWriter, PlistValue};
pub use plist_parser::PlistParser;
