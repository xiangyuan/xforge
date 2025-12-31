//! PBXTargetDependency - Dependencies between targets

use xforge_core::{ObjectId, PBXObject};

/// Target dependency
#[derive(Debug, Clone)]
pub struct PBXTargetDependency {
    id: ObjectId,
    pub target: Option<ObjectId>,
    pub target_proxy: Option<ObjectId>,
    pub name: Option<String>,
}

impl PBXTargetDependency {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            target: None,
            target_proxy: None,
            name: None,
        }
    }
    
    pub fn with_target(mut self, target: ObjectId) -> Self {
        self.target = Some(target);
        self
    }
    
    pub fn with_target_proxy(mut self, proxy: ObjectId) -> Self {
        self.target_proxy = Some(proxy);
        self
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Default for PBXTargetDependency {
    fn default() -> Self {
        Self::new()
    }
}

impl PBXObject for PBXTargetDependency {
    fn isa(&self) -> &'static str {
        "PBXTargetDependency"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}
