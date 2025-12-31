//! PBXFileSystemSynchronized types (Xcode 14+)

use xforge_core::{ObjectId, PBXObject};

/// PBXFileSystemSynchronizedBuildFileExceptionSet - Xcode 14+ file system synchronization exceptions
#[derive(Debug, Clone)]
pub struct PBXFileSystemSynchronizedBuildFileExceptionSet {
    id: ObjectId,
    pub membership_exceptions: Vec<String>,
    pub target: Option<ObjectId>,
}

impl PBXFileSystemSynchronizedBuildFileExceptionSet {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            membership_exceptions: Vec::new(),
            target: None,
        }
    }
}

impl PBXObject for PBXFileSystemSynchronizedBuildFileExceptionSet {
    fn isa(&self) -> &'static str {
        "PBXFileSystemSynchronizedBuildFileExceptionSet"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// PBXFileSystemSynchronizedRootGroup - Xcode 14+ file system synchronized group
#[derive(Debug, Clone)]
pub struct PBXFileSystemSynchronizedRootGroup {
    id: ObjectId,
    pub path: Option<String>,
    pub source_tree: String,
    pub exceptions: Vec<ObjectId>,
}

impl PBXFileSystemSynchronizedRootGroup {
    pub fn new(path: String) -> Self {
        Self {
            id: ObjectId::generate(),
            path: Some(path),
            source_tree: "<group>".to_string(),
            exceptions: Vec::new(),
        }
    }
}

impl PBXObject for PBXFileSystemSynchronizedRootGroup {
    fn isa(&self) -> &'static str {
        "PBXFileSystemSynchronizedRootGroup"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
