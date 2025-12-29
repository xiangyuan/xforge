//! Build configurations

use xforge_core::{ObjectId, PBXObject};
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct XCBuildConfiguration {
    pub id: ObjectId,
    pub name: String,
    pub build_settings: IndexMap<String, String>,
}

impl XCBuildConfiguration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            build_settings: IndexMap::new(),
        }
    }
}

impl PBXObject for XCBuildConfiguration {
    fn isa(&self) -> &'static str {
        "XCBuildConfiguration"
    }
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

#[derive(Debug, Clone)]
pub struct XCConfigurationList {
    pub id: ObjectId,
    pub build_configurations: Vec<ObjectId>,
}

impl XCConfigurationList {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            build_configurations: Vec::new(),
        }
    }
}

impl Default for XCConfigurationList {
    fn default() -> Self {
        Self::new()
    }
}

impl PBXObject for XCConfigurationList {
    fn isa(&self) -> &'static str {
        "XCConfigurationList"
    }
}
