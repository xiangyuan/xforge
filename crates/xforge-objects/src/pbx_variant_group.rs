//! PBXVariantGroup - Group for localized resources

use xforge_core::{ObjectId, Handle, PBXObject};
use crate::PBXFileReference;

/// Variant group for localized resources (e.g., Main.storyboard)
#[derive(Debug, Clone)]
pub struct PBXVariantGroup {
    id: ObjectId,
    pub name: Option<String>,
    pub children: Vec<Handle<PBXFileReference>>,
    pub source_tree: String,
}

impl PBXVariantGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: Some(name.into()),
            children: Vec::new(),
            source_tree: "<group>".to_string(),
        }
    }
    
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    
    pub fn children(&self) -> &[Handle<PBXFileReference>] {
        &self.children
    }
    
    pub fn add_child(&mut self, child: Handle<PBXFileReference>) {
        self.children.push(child);
    }
}

impl PBXObject for PBXVariantGroup {
    fn isa(&self) -> &'static str {
        "PBXVariantGroup"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_group() {
        let group = PBXVariantGroup::new("Main.storyboard");
        assert_eq!(group.isa(), "PBXVariantGroup");
        assert_eq!(group.name(), Some("Main.storyboard"));
        assert_eq!(group.children().len(), 0);
    }
}
