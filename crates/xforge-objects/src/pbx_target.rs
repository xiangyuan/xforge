//! PBXTarget and its variants

use xforge_core::{ObjectId, PBXObject};

#[derive(Debug, Clone)]
pub struct PBXNativeTarget {
    pub id: ObjectId,
    pub name: String,
    pub build_configuration_list: Option<ObjectId>,
    pub build_phases: Vec<ObjectId>,
    pub build_rules: Vec<ObjectId>,
    pub dependencies: Vec<ObjectId>,
    pub product_name: Option<String>,
    pub product_reference: Option<ObjectId>,
    pub product_type: Option<String>,
}

impl PBXNativeTarget {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            build_configuration_list: None,
            build_phases: Vec::new(),
            build_rules: Vec::new(),
            dependencies: Vec::new(),
            product_name: None,
            product_reference: None,
            product_type: None,
        }
    }
}

impl PBXObject for PBXNativeTarget {
    fn isa(&self) -> &'static str {
        "PBXNativeTarget"
    }
    
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}
