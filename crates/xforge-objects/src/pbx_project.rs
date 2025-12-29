//! PBXProject - The root object of an Xcode project

use xforge_core::{ObjectId, PBXObject};
use indexmap::IndexMap;

/// PBXProject - Root project object
#[derive(Debug, Clone)]
pub struct PBXProject {
    /// Unique identifier
    pub id: ObjectId,
    
    /// Project name
    pub name: String,
    
    /// Build configuration list
    pub build_configuration_list: Option<ObjectId>,
    
    /// Compatibility version (usually "Xcode 14.0")
    pub compatibility_version: String,
    
    /// Development region (e.g., "en")
    pub development_region: String,
    
    /// Has scanned for encodings
    pub has_scanned_for_encodings: bool,
    
    /// Known regions
    pub known_regions: Vec<String>,
    
    /// Main group
    pub main_group: ObjectId,
    
    /// Product ref group (optional)
    pub product_ref_group: Option<ObjectId>,
    
    /// Project dir path
    pub project_dir_path: String,
    
    /// Project root
    pub project_root: String,
    
    /// Targets
    pub targets: Vec<ObjectId>,
    
    /// Package references (Swift packages)
    pub package_references: Vec<ObjectId>,
    
    /// Project references (sub-projects)
    pub project_references: Vec<ProjectReference>,
    
    /// Attributes
    pub attributes: IndexMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ProjectReference {
    pub product_group: ObjectId,
    pub project_ref: ObjectId,
}

impl PBXProject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            build_configuration_list: None,
            compatibility_version: "Xcode 14.0".to_string(),
            development_region: "en".to_string(),
            has_scanned_for_encodings: false,
            known_regions: vec!["en".to_string(), "Base".to_string()],
            main_group: ObjectId::generate(),
            product_ref_group: None,
            project_dir_path: String::new(),
            project_root: String::new(),
            targets: Vec::new(),
            package_references: Vec::new(),
            project_references: Vec::new(),
            attributes: IndexMap::new(),
        }
    }
}

impl PBXObject for PBXProject {
    fn isa(&self) -> &'static str {
        "PBXProject"
    }
    
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbx_project_creation() {
        let project = PBXProject::new("TestProject");
        assert_eq!(project.name, "TestProject");
        assert_eq!(project.isa(), "PBXProject");
        assert_eq!(project.development_region, "en");
    }
}
