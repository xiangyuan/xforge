//! PBXLegacyTarget - Legacy build target for external build systems

use xforge_core::{Handle, ObjectId, PBXObject};
use crate::pbx_build_configuration::XCConfigurationList;
use crate::pbx_target_dependency::PBXTargetDependency;

/// Represents a legacy target that uses an external build system
/// (e.g., Makefile, custom build script)
#[derive(Debug, Clone)]
pub struct PBXLegacyTarget {
    id: ObjectId,
    pub name: String,
    pub product_name: Option<String>,
    pub build_configuration_list: Option<Handle<XCConfigurationList>>,
    pub build_tool_path: String,
    pub build_arguments_string: Option<String>,
    pub build_working_directory: Option<String>,
    pub pass_build_settings_in_environment: bool,
    pub dependencies: Vec<Handle<PBXTargetDependency>>,
}

impl PBXLegacyTarget {
    pub fn new(name: impl Into<String>, build_tool_path: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            product_name: None,
            build_configuration_list: None,
            build_tool_path: build_tool_path.into(),
            build_arguments_string: None,
            build_working_directory: None,
            pass_build_settings_in_environment: true,
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

    pub fn with_build_arguments(mut self, args: impl Into<String>) -> Self {
        self.build_arguments_string = Some(args.into());
        self
    }

    pub fn with_working_directory(mut self, dir: impl Into<String>) -> Self {
        self.build_working_directory = Some(dir.into());
        self
    }

    pub fn with_pass_build_settings(mut self, pass: bool) -> Self {
        self.pass_build_settings_in_environment = pass;
        self
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

    pub fn build_tool_path(&self) -> &str {
        &self.build_tool_path
    }

    pub fn build_arguments_string(&self) -> Option<&str> {
        self.build_arguments_string.as_deref()
    }

    pub fn build_working_directory(&self) -> Option<&str> {
        self.build_working_directory.as_deref()
    }

    pub fn pass_build_settings_in_environment(&self) -> bool {
        self.pass_build_settings_in_environment
    }

    pub fn dependencies(&self) -> &[Handle<PBXTargetDependency>] {
        &self.dependencies
    }
}

impl PBXObject for PBXLegacyTarget {
    fn isa(&self) -> &'static str {
        "PBXLegacyTarget"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_target_creation() {
        let target = PBXLegacyTarget::new("ExternalBuild", "/usr/bin/make");
        assert_eq!(target.name(), "ExternalBuild");
        assert_eq!(target.build_tool_path(), "/usr/bin/make");
        assert_eq!(target.isa(), "PBXLegacyTarget");
        assert!(target.pass_build_settings_in_environment());
    }

    #[test]
    fn test_legacy_target_with_arguments() {
        let target = PBXLegacyTarget::new("ExternalBuild", "/usr/bin/make")
            .with_build_arguments("clean all")
            .with_working_directory("$(SRCROOT)/external");
        assert_eq!(target.build_arguments_string(), Some("clean all"));
        assert_eq!(target.build_working_directory(), Some("$(SRCROOT)/external"));
    }
}
