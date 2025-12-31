//! PBXFileReference - References to files

use xforge_core::{ObjectId, PBXObject};

#[derive(Debug, Clone)]
pub struct PBXFileReference {
    pub id: ObjectId,
    pub path: Option<String>,
    pub name: Option<String>,
    pub last_known_file_type: Option<String>,
    pub source_tree: String,
    pub file_encoding: Option<u32>,
    pub explicit_file_type: Option<String>,
}

impl PBXFileReference {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            path: Some(path.into()),
            name: None,
            last_known_file_type: None,
            source_tree: "<group>".to_string(),
            file_encoding: None,
            explicit_file_type: None,
        }
    }
    
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    
    pub fn source_tree(&self) -> Option<&str> {
        Some(&self.source_tree)
    }
}

impl PBXObject for PBXFileReference {
    fn isa(&self) -> &'static str {
        "PBXFileReference"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}
