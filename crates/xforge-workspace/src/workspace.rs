use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::{FileRef, Group, Result};

/// Represents an Xcode workspace
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Workspace")]
pub struct Workspace {
    /// Workspace version
    #[serde(rename = "@version")]
    pub version: String,
    
    /// File references in the workspace
    #[serde(rename = "FileRef", default, skip_serializing_if = "Vec::is_empty")]
    pub file_refs: Vec<FileRef>,
    
    /// Groups in the workspace
    #[serde(rename = "Group", default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    /// Creates a new empty workspace
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            file_refs: Vec::new(),
            groups: Vec::new(),
        }
    }
    
    /// Loads a workspace from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let workspace_path = if path.extension().and_then(|s| s.to_str()) == Some("xcworkspace") {
            path.join("contents.xcworkspacedata")
        } else {
            path.to_path_buf()
        };
        
        let content = fs::read_to_string(&workspace_path)
            .map_err(|e| anyhow::anyhow!("Failed to read workspace file: {}", e))?;
        
        Self::from_xml(&content)
    }
    
    /// Saves the workspace to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let workspace_path = if path.extension().and_then(|s| s.to_str()) == Some("xcworkspace") {
            // Create .xcworkspace directory if it doesn't exist
            fs::create_dir_all(path)?;
            path.join("contents.xcworkspacedata")
        } else {
            path.to_path_buf()
        };
        
        let xml = self.to_xml()?;
        fs::write(&workspace_path, xml)
            .map_err(|e| anyhow::anyhow!("Failed to write workspace file: {}", e))?;
        
        Ok(())
    }
    
    /// Parses workspace from XML string
    pub fn from_xml(xml: &str) -> Result<Self> {
        quick_xml::de::from_str(xml)
            .map_err(|e| anyhow::anyhow!("Failed to parse workspace XML: {}", e))
    }
    
    /// Converts workspace to XML string
    pub fn to_xml(&self) -> Result<String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let serialized = quick_xml::se::to_string(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize workspace: {}", e))?;
        xml.push_str(&serialized);
        Ok(xml)
    }
    
    /// Adds a file reference to the workspace
    pub fn add_file_ref(&mut self, file_ref: FileRef) {
        self.file_refs.push(file_ref);
    }
    
    /// Adds a project to the workspace
    pub fn add_project<P: AsRef<Path>>(&mut self, project_path: P) -> Result<()> {
        let path = project_path.as_ref();
        let path_str = path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid project path"))?;
        
        self.add_file_ref(FileRef::container(path_str));
        Ok(())
    }
    
    /// Removes a file reference by path
    pub fn remove_file_ref(&mut self, path: &str) -> bool {
        let initial_len = self.file_refs.len();
        self.file_refs.retain(|f| f.path != path);
        self.file_refs.len() < initial_len
    }
    
    /// Adds a group to the workspace
    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }
    
    /// Gets all project paths
    pub fn project_paths(&self) -> Vec<&str> {
        self.file_refs.iter()
            .map(|f| f.path.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new();
        assert_eq!(workspace.version, "1.0");
        assert!(workspace.file_refs.is_empty());
        assert!(workspace.groups.is_empty());
    }

    #[test]
    fn test_add_file_ref() {
        let mut workspace = Workspace::new();
        workspace.add_file_ref(FileRef::container("MyApp.xcodeproj"));
        
        assert_eq!(workspace.file_refs.len(), 1);
        assert_eq!(workspace.file_refs[0].path, "MyApp.xcodeproj");
    }

    #[test]
    fn test_add_project() {
        let mut workspace = Workspace::new();
        workspace.add_project("MyApp.xcodeproj").unwrap();
        
        assert_eq!(workspace.file_refs.len(), 1);
        assert_eq!(workspace.project_paths(), vec!["MyApp.xcodeproj"]);
    }

    #[test]
    fn test_remove_file_ref() {
        let mut workspace = Workspace::new();
        workspace.add_file_ref(FileRef::container("App1.xcodeproj"));
        workspace.add_file_ref(FileRef::container("App2.xcodeproj"));
        
        assert!(workspace.remove_file_ref("App1.xcodeproj"));
        assert_eq!(workspace.file_refs.len(), 1);
        assert_eq!(workspace.file_refs[0].path, "App2.xcodeproj");
    }

    #[test]
    fn test_xml_serialization() {
        let mut workspace = Workspace::new();
        workspace.add_file_ref(FileRef::container("MyApp.xcodeproj"));
        
        let xml = workspace.to_xml().unwrap();
        assert!(xml.contains("<?xml version"));
        assert!(xml.contains("Workspace"));
        assert!(xml.contains("MyApp.xcodeproj"));
        
        let parsed = Workspace::from_xml(&xml).unwrap();
        assert_eq!(parsed.file_refs.len(), 1);
    }

    #[test]
    fn test_roundtrip() {
        let mut workspace = Workspace::new();
        workspace.add_file_ref(FileRef::container("Project1.xcodeproj"));
        workspace.add_file_ref(FileRef::group("../Project2.xcodeproj"));
        
        let xml = workspace.to_xml().unwrap();
        let parsed = Workspace::from_xml(&xml).unwrap();
        
        assert_eq!(workspace, parsed);
    }
}
