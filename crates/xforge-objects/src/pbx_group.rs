//! PBXGroup - File groups

use xforge_core::{ObjectId, PBXObject};

#[derive(Debug, Clone)]
pub struct PBXGroup {
    pub id: ObjectId,
    pub name: Option<String>,
    pub path: Option<String>,
    pub children: Vec<ObjectId>,
    pub source_tree: String,
}

impl PBXGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: Some(name.into()),
            path: None,
            children: Vec::new(),
            source_tree: "<group>".to_string(),
        }
    }
}

impl PBXObject for PBXGroup {
    fn isa(&self) -> &'static str {
        "PBXGroup"
    }
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
