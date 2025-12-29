//! Builder for creating PBX targets

use xforge_model::{Platform, ProductType};
use xforge_objects::PBXNativeTarget;

/// Builder for creating PBX targets with a fluent API
pub struct TargetBuilder {
    name: String,
    platform: Option<Platform>,
    product_type: Option<ProductType>,
    product_name: Option<String>,
}

impl TargetBuilder {
    /// Create a new target builder with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            platform: None,
            product_type: None,
            product_name: None,
        }
    }
    
    /// Set the target platform
    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }
    
    /// Set the product type
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }
    
    /// Set the product name (defaults to target name if not specified)
    pub fn product_name(mut self, product_name: impl Into<String>) -> Self {
        self.product_name = Some(product_name.into());
        self
    }
    
    /// Build the target
    pub fn build(self) -> PBXNativeTarget {
        let mut target = PBXNativeTarget::new(&self.name);
        
        if let Some(product_name) = self.product_name {
            target.set_product_name(&product_name);
        }
        
        if let Some(product_type) = self.product_type {
            target.set_product_type(product_type);
        }
        
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xforge_core::PBXObject;

    #[test]
    fn test_target_builder_basic() {
        let target = TargetBuilder::new("MyApp").build();
        
        assert_eq!(target.name(), "MyApp");
    }

    #[test]
    fn test_target_builder_with_product() {
        let target = TargetBuilder::new("MyApp")
            .platform(Platform::iOS)
            .product_type(ProductType::Application)
            .product_name("MyApp.app")
            .build();
        
        assert_eq!(target.name(), "MyApp");
        assert_eq!(target.product_name.as_deref(), Some("MyApp.app"));
    }

    #[test]
    fn test_target_builder_framework() {
        let target = TargetBuilder::new("MyFramework")
            .platform(Platform::iOS)
            .product_type(ProductType::Framework)
            .product_name("MyFramework.framework")
            .build();
        
        assert_eq!(target.name(), "MyFramework");
        assert_eq!(target.product_type(), Some(ProductType::Framework));
    }
}
