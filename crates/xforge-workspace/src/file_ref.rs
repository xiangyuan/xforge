use serde::{Deserialize, Serialize};

/// Location type for file references
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileRefLocation {
    /// Absolute path
    Absolute,
    /// Relative to workspace
    Container,
    /// Relative to group
    Group,
    /// Self reference
    #[serde(rename = "self")]
    SelfRef,
}

impl Default for FileRefLocation {
    fn default() -> Self {
        Self::Container
    }
}

/// Represents a file reference in a workspace
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRef {
    /// Location type
    #[serde(rename = "@location")]
    pub location: FileRefLocation,
    
    /// File path
    #[serde(rename = "$text")]
    pub path: String,
}

impl FileRef {
    /// Creates a new file reference
    pub fn new(path: impl Into<String>, location: FileRefLocation) -> Self {
        Self {
            location,
            path: path.into(),
        }
    }
    
    /// Creates a container-relative file reference
    pub fn container(path: impl Into<String>) -> Self {
        Self::new(path, FileRefLocation::Container)
    }
    
    /// Creates a group-relative file reference
    pub fn group(path: impl Into<String>) -> Self {
        Self::new(path, FileRefLocation::Group)
    }
    
    /// Creates an absolute file reference
    pub fn absolute(path: impl Into<String>) -> Self {
        Self::new(path, FileRefLocation::Absolute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_ref_creation() {
        let file_ref = FileRef::container("MyApp.xcodeproj");
        assert_eq!(file_ref.path, "MyApp.xcodeproj");
        assert_eq!(file_ref.location, FileRefLocation::Container);
    }

    #[test]
    fn test_file_ref_types() {
        let container = FileRef::container("project.xcodeproj");
        let group = FileRef::group("../other.xcodeproj");
        let absolute = FileRef::absolute("/Users/dev/project.xcodeproj");
        
        assert_eq!(container.location, FileRefLocation::Container);
        assert_eq!(group.location, FileRefLocation::Group);
        assert_eq!(absolute.location, FileRefLocation::Absolute);
    }
}
