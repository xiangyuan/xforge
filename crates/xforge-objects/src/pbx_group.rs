//! PBXGroup - File groups

use xforge_core::{ObjectId, Handle, PBXObject};

#[derive(Debug, Clone)]
pub struct PBXGroup {
    id: ObjectId,
    pub name: Option<String>,
    pub path: Option<String>,
    pub children: Vec<ObjectId>,  // Changed from Vec<Handle<PBXFileReference>> to support any child type
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
    
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    
    pub fn source_tree(&self) -> Option<&str> {
        Some(&self.source_tree)
    }
    
    pub fn children(&self) -> &[ObjectId] {
        &self.children
    }
    
    pub fn add_child(&mut self, child: ObjectId) {
        self.children.push(child);
    }
}

impl PBXObject for PBXGroup {
    fn isa(&self) -> &'static str {
        "PBXGroup"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}

use crate::PBXFileReference;
