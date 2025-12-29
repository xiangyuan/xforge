//! High-level Project API for loading and saving Xcode projects

use std::fs;
use std::path::Path;
use xforge_core::{ObjectId, Registry};
use xforge_serialization::{PlistParser, PlistWriter};
use crate::{serialization, deserialization};

/// Represents a loaded Xcode project
pub struct Project {
    /// The object registry containing all project objects
    pub registry: Registry,
    /// The root project object ID
    pub root_id: ObjectId,
}

impl Project {
    /// Create a new empty project
    pub fn new(name: impl Into<String>) -> Self {
        let mut registry = Registry::new();
        let project = crate::pbx_project::PBXProject::new(name);
        
        let root_id = ObjectId::generate();
        registry.register_with_id(root_id.clone(), project);
        
        Self {
            registry,
            root_id,
        }
    }
    
    /// Load a project from a .pbxproj file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        
        // Read file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Parse plist
        let mut parser = PlistParser::new(&content);
        let plist = parser.parse()
            .map_err(|e| format!("Failed to parse plist: {}", e))?;
        
        // Deserialize into registry
        let (registry, root_id) = deserialization::deserialize_registry(&plist)?;
        
        Ok(Self {
            registry,
            root_id,
        })
    }
    
    /// Save the project to a .pbxproj file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();
        
        // Serialize registry to plist
        let plist = serialization::serialize_registry(&self.registry, &self.root_id.to_string());
        
        // Write to file
        let mut writer = PlistWriter::new();
        let content = writer.write_plist(&plist)?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }
    
    /// Get the root project object
    pub fn root_project(&self) -> Option<&crate::pbx_project::PBXProject> {
        self.registry.get::<crate::pbx_project::PBXProject>(&self.root_id)
    }
    
    /// Get a mutable reference to the root project object
    pub fn root_project_mut(&mut self) -> Option<&mut crate::pbx_project::PBXProject> {
        self.registry.get_mut::<crate::pbx_project::PBXProject>(&self.root_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_project() {
        let project = Project::new("TestProject");
        assert!(project.root_project().is_some());
    }
    
    #[test]
    fn test_project_name() {
        let project = Project::new("MyApp");
        if let Some(root) = project.root_project() {
            assert_eq!(root.name, "MyApp");
        }
    }
}
