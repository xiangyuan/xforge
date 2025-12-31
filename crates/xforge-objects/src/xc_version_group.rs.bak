//! XCVersionGroup - Version group for Core Data models

use xforge_core::{Handle, ObjectId, PBXObject};
use crate::pbx_file_reference::PBXFileReference;

/// Represents a versioned group of files, typically used for Core Data models
/// (e.g., MyModel.xcdatamodeld with multiple .xcdatamodel versions)
#[derive(Debug, Clone)]
pub struct XCVersionGroup {
    id: ObjectId,
    pub path: String,
    pub source_tree: String,
    pub children: Vec<Handle<PBXFileReference>>,
    pub current_version: Option<Handle<PBXFileReference>>,
    pub version_group_type: String,
}

impl XCVersionGroup {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            path: path.into(),
            source_tree: "<group>".to_string(),
            children: Vec::new(),
            current_version: None,
            version_group_type: "wrapper.xcdatamodel".to_string(),
        }
    }

    pub fn with_source_tree(mut self, source_tree: impl Into<String>) -> Self {
        self.source_tree = source_tree.into();
        self
    }

    pub fn with_version_group_type(mut self, group_type: impl Into<String>) -> Self {
        self.version_group_type = group_type.into();
        self
    }

    pub fn add_child(&mut self, child: Handle<PBXFileReference>) {
        self.children.push(child);
    }

    pub fn set_current_version(&mut self, version: Handle<PBXFileReference>) {
        self.current_version = Some(version);
    }

    // Getters
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn source_tree(&self) -> &str {
        &self.source_tree
    }

    pub fn children(&self) -> &[Handle<PBXFileReference>] {
        &self.children
    }

    pub fn current_version(&self) -> Option<&Handle<PBXFileReference>> {
        self.current_version.as_ref()
    }

    pub fn version_group_type(&self) -> &str {
        &self.version_group_type
    }
}

impl PBXObject for XCVersionGroup {
    fn isa(&self) -> &'static str {
        "XCVersionGroup"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_group_creation() {
        let group = XCVersionGroup::new("MyModel.xcdatamodeld");
        assert_eq!(group.path(), "MyModel.xcdatamodeld");
        assert_eq!(group.source_tree(), "<group>");
        assert_eq!(group.version_group_type(), "wrapper.xcdatamodel");
        assert_eq!(group.isa(), "XCVersionGroup");
        assert!(group.children().is_empty());
        assert!(group.current_version().is_none());
    }

    #[test]
    fn test_version_group_with_type() {
        let group = XCVersionGroup::new("MyMapping.xcmappingmodel")
            .with_version_group_type("wrapper.xcmappingmodel");
        assert_eq!(group.version_group_type(), "wrapper.xcmappingmodel");
    }
}
