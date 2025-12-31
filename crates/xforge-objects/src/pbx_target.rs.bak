//! PBXTarget and its variants

use xforge_core::{ObjectId, PBXObject, ProductType};
#[derive(Debug, Clone)]
pub struct PBXNativeTarget {
    id: ObjectId,
    pub name: String,
    pub build_configuration_list: Option<ObjectId>,
    pub build_phases: Vec<ObjectId>,
    pub build_rules: Vec<ObjectId>,
    pub dependencies: Vec<ObjectId>,
    pub package_product_dependencies: Vec<ObjectId>,
    pub product_name: Option<String>,
    pub product_reference: Option<ObjectId>,
    pub product_type: Option<ProductType>,
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
            package_product_dependencies: Vec::new(),
            product_name: None,
            product_reference: None,
            product_type: None,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn product_name(&self) -> Option<&str> {
        self.product_name.as_deref()
    }
    
    pub fn product_type(&self) -> Option<ProductType> {
        self.product_type.clone()
    }
    
    pub fn build_phases(&self) -> &[ObjectId] {
        &self.build_phases
    }
    
    pub fn set_product_name(&mut self, name: &str) {
        self.product_name = Some(name.to_string());
    }
    
    pub fn set_product_type(&mut self, product_type: ProductType) {
        self.product_type = Some(product_type);
    }
    
    pub fn add_build_phase(&mut self, phase_id: ObjectId) {
        self.build_phases.push(phase_id);
    }
    
    pub fn dependencies(&self) -> &[ObjectId] {
        &self.dependencies
    }
    
    pub fn package_product_dependencies(&self) -> &[ObjectId] {
        &self.package_product_dependencies
    }
}

impl PBXObject for PBXNativeTarget {
    fn isa(&self) -> &'static str {
        "PBXNativeTarget"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}
