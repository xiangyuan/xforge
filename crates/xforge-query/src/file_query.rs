//! File and group query API

use xforge_core::{Handle, Registry, ObjectId};
use xforge_objects::{PBXGroup, PBXFileReference};

/// Query API for file and group operations
pub struct FileQuery<'a> {
    registry: &'a Registry,
}

impl<'a> FileQuery<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn files_in_group(&self, group_id: &ObjectId) -> Vec<&PBXFileReference> {
        let mut files = Vec::new();
        if let Some(group_obj) = self.registry.get::<PBXGroup>(group_id) {
            for child_handle in group_obj.children() {
                if let Some(file) = self.registry.get::<PBXFileReference>(child_handle.id()) {
                    files.push(file);
                }
            }
        }
        files
    }

    pub fn subgroups_in_group(&self, group_id: &ObjectId) -> Vec<&PBXGroup> {
        let mut groups = Vec::new();
        if let Some(group_obj) = self.registry.get::<PBXGroup>(group_id) {
            for child_handle in group_obj.children() {
                if let Some(subgroup) = self.registry.get::<PBXGroup>(child_handle.id()) {
                    groups.push(subgroup);
                }
            }
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_query_creation() {
        let registry = Registry::new();
        let _query = FileQuery::new(&registry);
    }
}
