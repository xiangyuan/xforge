//! PBXFileSystemSynchronized types (Xcode 14+)

use indexmap::IndexMap;
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

    fn id(&self) -> &ObjectId {
        &self.id
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

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet - Xcode 15+ group build phase membership exceptions
#[derive(Debug, Clone)]
pub struct PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet {
    id: ObjectId,
    pub build_phase: ObjectId,
    pub membership_exceptions: Vec<String>,
    pub attributes_by_relative_path: IndexMap<String, Vec<String>>,
}

impl PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet {
    pub fn new(build_phase: ObjectId) -> Self {
        Self {
            id: ObjectId::generate(),
            build_phase,
            membership_exceptions: Vec::new(),
            attributes_by_relative_path: IndexMap::new(),
        }
    }
}

impl PBXObject for PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet {
    fn isa(&self) -> &'static str {
        "PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
