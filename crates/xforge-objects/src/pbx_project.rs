//! PBXProject - The root object of an Xcode project

use xforge_core::{ObjectId, PBXObject};
use crate::versioning::{compatibility_version_for, DEFAULT_OBJECT_VERSION};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;

/// PBXProject - Root project object
#[derive(Debug, Clone)]
pub struct PBXProject {
    id: ObjectId,
    pub name: String,
    pub build_configuration_list: Option<ObjectId>,
    pub compatibility_version: String,
    pub development_region: String,
    pub has_scanned_for_encodings: bool,
    pub known_regions: Vec<String>,
    pub main_group: Option<ObjectId>,
    pub product_ref_group: Option<ObjectId>,
    pub project_dir_path: String,
    pub project_root: String,
    pub targets: Vec<ObjectId>,
    pub package_references: Vec<ObjectId>,
    pub project_references: Vec<ProjectReference>,
    pub attributes: IndexMap<String, PlistValue>,
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
            compatibility_version: compatibility_version_for(DEFAULT_OBJECT_VERSION)
                .unwrap_or("Xcode 14.0")
                .to_string(),
            development_region: "en".to_string(),
            has_scanned_for_encodings: false,
            known_regions: vec!["en".to_string(), "Base".to_string()],
            main_group: None,
            product_ref_group: None,
            project_dir_path: String::new(),
            project_root: String::new(),
            targets: Vec::new(),
            package_references: Vec::new(),
            project_references: Vec::new(),
            attributes: IndexMap::new(),
        }
    }
    
    pub fn compatibility_version(&self) -> &str {
        &self.compatibility_version
    }
    
    pub fn development_region(&self) -> &str {
        &self.development_region
    }
    
    pub fn main_group(&self) -> Option<ObjectId> {
        self.main_group
    }
    
    pub fn targets(&self) -> &[ObjectId] {
        &self.targets
    }
    
    pub fn add_target(&mut self, target_id: ObjectId) {
        self.targets.push(target_id);
    }
}

impl PBXObject for PBXProject {
    fn isa(&self) -> &'static str {
        "PBXProject"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbx_project_creation() {
        let project = PBXProject::new("TestProject");
        assert_eq!(project.name, "TestProject");
        assert_eq!(project.isa(), "PBXProject");
        assert_eq!(project.development_region(), "en");
    }
}
