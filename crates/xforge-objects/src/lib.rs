//! xforge-objects - PBX object definitions

pub mod pbx_project;
pub mod pbx_target;
pub mod pbx_file_reference;
pub mod pbx_group;
pub mod pbx_build_configuration;
pub mod pbx_build_phase;

pub use pbx_project::{PBXProject, ProjectReference};
pub use pbx_target::PBXNativeTarget;
pub use pbx_file_reference::PBXFileReference;
pub use pbx_group::PBXGroup;
pub use pbx_build_configuration::{
    XCBuildConfiguration, 
    XCConfigurationList,
};
pub use pbx_build_phase::{
    PBXSourcesBuildPhase,
    PBXFrameworksBuildPhase,
    PBXResourcesBuildPhase,
    PBXShellScriptBuildPhase,
    PBXCopyFilesBuildPhase,
    PBXBuildFile,
};

#[cfg(test)]
mod tests {
    use super::*;
    use xforge_core::PBXObject;

    #[test]
    fn test_all_objects_have_isa() {
        let project = PBXProject::new("Test");
        assert_eq!(project.isa(), "PBXProject");
        
        let target = PBXNativeTarget::new("Test");
        assert_eq!(target.isa(), "PBXNativeTarget");
        
        let file = PBXFileReference::new("test.swift");
        assert_eq!(file.isa(), "PBXFileReference");
        
        let group = PBXGroup::new("Sources");
        assert_eq!(group.isa(), "PBXGroup");
        
        let config = XCBuildConfiguration::new("Debug");
        assert_eq!(config.isa(), "XCBuildConfiguration");
        
        let sources = PBXSourcesBuildPhase::new();
        assert_eq!(sources.isa(), "PBXSourcesBuildPhase");
        
        let frameworks = PBXFrameworksBuildPhase::new();
        assert_eq!(frameworks.isa(), "PBXFrameworksBuildPhase");
        
        let resources = PBXResourcesBuildPhase::new();
        assert_eq!(resources.isa(), "PBXResourcesBuildPhase");
    }
}
