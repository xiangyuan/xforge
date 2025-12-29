//! PBXAggregateTarget - Aggregate build target

use xforge_core::{Handle, ObjectId, PBXObject};
use crate::pbx_build_phase::PBXShellScriptBuildPhase;
use crate::pbx_build_configuration::XCConfigurationList;
use crate::pbx_target_dependency::PBXTargetDependency;

/// Represents an aggregate target that doesn't produce a binary product
/// but can be used to run scripts or aggregate other targets
#[derive(Debug, Clone)]
pub struct PBXAggregateTarget {
    id: ObjectId,
    pub name: String,
    pub product_name: Option<String>,
    pub build_configuration_list: Option<Handle<XCConfigurationList>>,
    pub build_phases: Vec<Handle<PBXShellScriptBuildPhase>>,
    pub dependencies: Vec<Handle<PBXTargetDependency>>,
}

impl PBXAggregateTarget {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            product_name: None,
            build_configuration_list: None,
            build_phases: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn with_product_name(mut self, product_name: impl Into<String>) -> Self {
        self.product_name = Some(product_name.into());
        self
    }

    pub fn with_build_configuration_list(mut self, list: Handle<XCConfigurationList>) -> Self {
        self.build_configuration_list = Some(list);
        self
    }

    pub fn add_build_phase(&mut self, phase: Handle<PBXShellScriptBuildPhase>) {
        self.build_phases.push(phase);
    }

    pub fn add_dependency(&mut self, dependency: Handle<PBXTargetDependency>) {
        self.dependencies.push(dependency);
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn product_name(&self) -> Option<&str> {
        self.product_name.as_deref()
    }

    pub fn build_configuration_list(&self) -> Option<&Handle<XCConfigurationList>> {
        self.build_configuration_list.as_ref()
    }

    pub fn build_phases(&self) -> &[Handle<PBXShellScriptBuildPhase>] {
        &self.build_phases
    }

    pub fn dependencies(&self) -> &[Handle<PBXTargetDependency>] {
        &self.dependencies
    }
}

impl PBXObject for PBXAggregateTarget {
    fn isa(&self) -> &'static str {
        "PBXAggregateTarget"
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
    fn test_aggregate_target_creation() {
        let target = PBXAggregateTarget::new("BuildAll");
        assert_eq!(target.name(), "BuildAll");
        assert_eq!(target.isa(), "PBXAggregateTarget");
        assert!(target.build_phases().is_empty());
        assert!(target.dependencies().is_empty());
    }

    #[test]
    fn test_aggregate_target_with_product_name() {
        let target = PBXAggregateTarget::new("BuildAll")
            .with_product_name("BuildAll");
        assert_eq!(target.product_name(), Some("BuildAll"));
    }
}
