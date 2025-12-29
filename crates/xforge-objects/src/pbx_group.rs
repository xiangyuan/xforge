//! PBXGroup - File groups

use xforge_core::{ObjectId, Handle, PBXObject};

#[derive(Debug, Clone)]
pub struct PBXGroup {
    id: ObjectId,
    pub name: Option<String>,
    pub path: Option<String>,
    pub children: Vec<Handle<PBXFileReference>>,
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
    
    pub fn children(&self) -> &[Handle<PBXFileReference>] {
        &self.children
    }
    
    pub fn add_child(&mut self, child: Handle<PBXFileReference>) {
        self.children.push(child);
    }
}

impl PBXObject for PBXGroup {
    fn isa(&self) -> &'static str {
        "PBXGroup"
    }
}

use crate::PBXFileReference;
