//! Project - Core domain model for Xcode projects

use xforge_core::{ObjectId, Registry};
use std::path::{Path, PathBuf};

/// Xcode project
pub struct Project {
    path: PathBuf,
    registry: Registry,
    root_id: ObjectId,
    metadata: ProjectMetadata,
}

/// Project metadata
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub archive_version: String,
    pub object_version: String,
    pub name: String,
    pub organization: Option<String>,
    pub development_region: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            archive_version: "1".to_string(),
            object_version: "56".to_string(),
            name: "Project".to_string(),
            organization: None,
            development_region: "en".to_string(),
        }
    }
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let path = PathBuf::from(format!("{}.xcodeproj", name));
        let registry = Registry::new();
        let root_id = ObjectId::generate();
        
        let mut metadata = ProjectMetadata::default();
        metadata.name = name;
        
        Self { path, registry, root_id, metadata }
    }
    
    pub fn name(&self) -> &str {
        &self.metadata.name
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = path.into();
    }
    
    pub fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }
    
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    
    pub fn metadata(&self) -> &ProjectMetadata {
        &self.metadata
    }
    
    pub fn metadata_mut(&mut self) -> &mut ProjectMetadata {
        &mut self.metadata
    }
    
    pub fn root_id(&self) -> ObjectId {
        self.root_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("TestProject");
        assert_eq!(project.name(), "TestProject");
        assert!(project.registry().is_empty());
    }
}
