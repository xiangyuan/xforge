use serde::{Deserialize, Serialize};
use crate::FileRef;

/// Represents a group in a workspace
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    /// Group location
    #[serde(rename = "@location")]
    pub location: String,
    
    /// Group name
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// File references in this group
    #[serde(rename = "FileRef", default, skip_serializing_if = "Vec::is_empty")]
    pub file_refs: Vec<FileRef>,
    
    /// Nested groups
    #[serde(rename = "Group", default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
}

impl Group {
    /// Creates a new group
    pub fn new(location: impl Into<String>, name: Option<String>) -> Self {
        Self {
            location: location.into(),
            name,
            file_refs: Vec::new(),
            groups: Vec::new(),
        }
    }
    
    /// Adds a file reference to this group
    pub fn add_file_ref(&mut self, file_ref: FileRef) {
        self.file_refs.push(file_ref);
    }
    
    /// Adds a nested group
    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FileRefLocation;

    #[test]
    fn test_group_creation() {
        let mut group = Group::new("container", Some("Projects".to_string()));
        assert_eq!(group.location, "container");
        assert_eq!(group.name, Some("Projects".to_string()));
        
        group.add_file_ref(FileRef::container("App.xcodeproj"));
        assert_eq!(group.file_refs.len(), 1);
    }

    #[test]
    fn test_nested_groups() {
        let mut parent = Group::new("container", Some("Parent".to_string()));
        let child = Group::new("group", Some("Child".to_string()));
        
        parent.add_group(child);
        assert_eq!(parent.groups.len(), 1);
    }
}
