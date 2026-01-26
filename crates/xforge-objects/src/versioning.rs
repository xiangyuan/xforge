//! Xcode project file versioning utilities.

use indexmap::IndexMap;
use xforge_serialization::PlistValue;

/// Last known archive version (Xcodeproj uses 1).
pub const LAST_KNOWN_ARCHIVE_VERSION: i64 = 1;

/// Default object version for new projects.
pub const DEFAULT_OBJECT_VERSION: i64 = 56;

/// Last known object version (from CocoaPods/Xcodeproj).
pub const LAST_KNOWN_OBJECT_VERSION: i64 = 77;

/// Maps objectVersion to compatibilityVersion.
pub fn compatibility_version_for(object_version: i64) -> Option<&'static str> {
    match object_version {
        77 => Some("Xcode 16.0"),
        71 => Some("Xcode 16.2"),
        70 => Some("Xcode 16.0"),
        63 => Some("Xcode 15.3"),
        60 => Some("Xcode 15.0"),
        56 => Some("Xcode 14.0"),
        55 => Some("Xcode 13.0"),
        54 => Some("Xcode 12.0"),
        53 => Some("Xcode 11.4"),
        52 => Some("Xcode 11.0"),
        51 => Some("Xcode 10.0"),
        50 => Some("Xcode 9.3"),
        48 => Some("Xcode 8.0"),
        47 => Some("Xcode 6.3"),
        46 => Some("Xcode 3.2"),
        45 => Some("Xcode 3.1"),
        _ => None,
    }
}

/// Root-level file format metadata.
#[derive(Debug, Clone)]
pub struct ProjectFileFormat {
    pub archive_version: i64,
    pub object_version: i64,
    pub classes: IndexMap<String, PlistValue>,
    pub root_unknown_fields: IndexMap<String, PlistValue>,
}

impl Default for ProjectFileFormat {
    fn default() -> Self {
        Self {
            archive_version: LAST_KNOWN_ARCHIVE_VERSION,
            object_version: DEFAULT_OBJECT_VERSION,
            classes: IndexMap::new(),
            root_unknown_fields: IndexMap::new(),
        }
    }
}
